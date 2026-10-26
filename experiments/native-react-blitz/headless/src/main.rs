//! Runtime smoke test: genuine React/Preact JS execution in Boa/Blitz.
//! Does not validate graphics, accessibility, CSS, or packaging.
use blitz_dom::{Document, DocumentConfig};
use blitz_traits::events::{DomEvent, UiEvent, BlitzImeEvent};
use blitz_vibey_script::ScriptDocument;
use blitz_traits::shell::{Viewport, ColorScheme};
use keyboard_types::Modifiers;
use std::{env,fs,path::PathBuf,process};
use url::Url;
use anyrender::{PaintScene as _, render_to_buffer};
use peniko::{Fill,kurbo::Rect};
use blitz_dom::util::Color;
use anyrender_vello_cpu::VelloCpuImageRenderer;
use blitz_paint::paint_scene;
use png;
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;

fn pump(doc: &mut ScriptDocument) {
    for _ in 0..30 {
        if let Some(next) = doc.next_timer_deadline() {
            doc.advance_clock_to(next);
        }
        doc.poll(None);
    }
}
fn text(doc: &ScriptDocument) -> String {
    let borrowed = doc.inner();
    borrowed.get_element_by_id("app")
        .and_then(|id| borrowed.get_node(id))
        .map(|node| node.text_content())
        .unwrap_or_default()
}

fn paint_png(doc: &mut ScriptDocument, path: &str) -> Result<(), String> {
    // No GPU surface: use Blitz's actual CSS/layout + Vello CPU paint pipeline.
    let width = 980u32;
    let height = 660u32;
    doc.inner_mut().resolve(0.0);
    let pixels = render_to_buffer::<VelloCpuImageRenderer, _>(
        |scene| {
            scene.fill(
                Fill::NonZero,
                Default::default(),
                Color::WHITE,
                Default::default(),
                &Rect::new(0.0, 0.0, width as f64, height as f64),
            );
            let mut inner = doc.inner_mut();
            paint_scene(scene, &mut inner, 1.0, width, height, 0, 0);
        },
        width,
        height,
    );
    let dark_pixels = pixels.chunks_exact(4).filter(|p| p[0] < 240 || p[1] < 240 || p[2] < 240).count();
    let coverage = dark_pixels as f64 / (width as f64 * height as f64);
    println!("NATIVE_CPU_PAINT_NONWHITE_RATIO={coverage:.6}");
    if coverage < 0.0001 { return Err(format!("paint appears blank: nonwhite coverage {coverage:.6}")); }
    let file = File::create(path).map_err(|e| e.to_string())?;
    let mut encoder = png::Encoder::new(BufWriter::new(file), width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
    writer.write_image_data(&pixels).map_err(|e| e.to_string())?;
    println!("NATIVE_CPU_PNG_PATH={path}");
    println!("NATIVE_CPU_PNG_BYTES={}", pixels.len());
    Ok(())
}

fn run(mode: &str, path: PathBuf, png_path: Option<&str>) -> Result<(), String> {
    let path = path.canonicalize().map_err(|e| format!("No HTML {}: {e}", path.display()))?;
    let html = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let url = Url::from_file_path(&path).map_err(|_| "Invalid local file path")?;
    let mut doc = ScriptDocument::from_html(
        &html,
        DocumentConfig { base_url: Some(url.to_string()), viewport: Some(Viewport::new(980,660,1.0,ColorScheme::Light)), ..Default::default() },
    ).without_timer_thread().with_virtual_time();
    doc.execute_scripts();
    pump(&mut doc);
    let before = text(&doc);
    println!("NATIVE_PROBE_MODE={mode}");
    println!("NATIVE_PROBE_BEFORE={before:?}");
    let errors = doc.take_js_errors();
    for error in &errors { eprintln!("NATIVE_PROBE_JS_ERROR={error}"); }
    if !errors.is_empty() { return Err(format!("{} Boa JS exceptions", errors.len())); }
    let expected = match mode {
       "preact" => "todos",
       "react" => "Count: 0",
       "astryx" | "astryx-input" | "react-input" | "stress" => "Count: 0",
       _ => return Err(format!("unknown mode {mode}"))
    };
    if !before.contains(expected) {
        return Err(format!("Missing {expected:?} from executed UI; received {before:?}"));
    }
    if mode == "astryx" || mode == "react" {
        let click = {
            let borrowed = doc.inner();
            let id = borrowed.query_selector("button").map_err(|e| format!("button query: {e:?}"))?
                .ok_or("Missing Astryx button")?;
            let button = borrowed.get_node(id).ok_or("Button node missing")?;
            DomEvent::new(id, button.synthetic_click_event(Modifiers::empty()))
        };
        doc.dispatch_dom_event(click);
        pump(&mut doc);
        let after = text(&doc);
        println!("NATIVE_PROBE_AFTER_CLICK={after:?}");
        let errors = doc.take_js_errors();
        for error in &errors { eprintln!("NATIVE_PROBE_JS_ERROR_AFTER={error}"); }
        if !errors.is_empty() || !after.contains("Count: 1") {
            return Err("Native event failed to update Astryx React state".into());
        }
    }
    if mode == "astryx-input" || mode == "react-input" {
        // Native input widgets are created during Blitz layout construction.
        // DOM-only script execution cannot exercise a text editor without a resolve.
        doc.inner_mut().resolve(0.0);
        let input_id = {
            let borrowed = doc.inner();
            borrowed.query_selector("input").map_err(|e| format!("input selector: {e:?}"))?
                .ok_or("Missing input control")?
        };
        {
            let mut borrowed = doc.inner_mut();
            let focused = borrowed.set_focus_to(input_id);
            println!("NATIVE_PROBE_FOCUS_CHANGED={focused}");
        }
        {
            let borrowed = doc.inner();
            let native_editor = borrowed.get_node(input_id).and_then(|node| node.element_data())
                .and_then(|el| el.text_input_data())
                .map(|data| data.editor.raw_text().to_string());
            println!("NATIVE_PROBE_NATIVE_EDITOR_BEFORE={native_editor:?}");
            println!("NATIVE_PROBE_FOCUSED_NODE={:?}", borrowed.get_focussed_node_id());
        }
        // Simulate a real input-method commit through native Blitz event handling;
        // do not directly call the React setState callback or inject a fake DOM.
        doc.handle_ui_event(UiEvent::Ime(BlitzImeEvent::Commit("native hello".into())));
        pump(&mut doc);
        let input_after = text(&doc);
        {
            let borrowed = doc.inner();
            let editor = borrowed.get_node(input_id).and_then(|node| node.element_data())
                .and_then(|el| el.text_input_data())
                .map(|data| data.editor.raw_text().to_string());
            println!("NATIVE_PROBE_NATIVE_EDITOR_AFTER={editor:?}");
        }
        println!("NATIVE_PROBE_AFTER_NATIVE_IME={input_after:?}");
        let errors = doc.take_js_errors();
        for error in &errors { eprintln!("NATIVE_PROBE_JS_ERROR_INPUT={error}"); }
        if !errors.is_empty() || !input_after.contains("Echo: native hello") {
            return Err("Native IME input did not update React's controlled input".into());
        }
    }
    if let Some(path) = png_path { paint_png(&mut doc,path)?; }
    println!("NATIVE_REACT_{mode}_PASS");
    Ok(())
}

fn millis(time: std::time::Duration) -> f64 { time.as_secs_f64() * 1000.0 }

// One fresh Boa context and React tree per invocation. The external benchmark
// samples many fresh processes and reports process wall/peak RSS separately.
fn bench(mode: &str, path: PathBuf) -> Result<(), String> {
    let path = path.canonicalize().map_err(|e| format!("Invalid HTML: {e}"))?;
    let html = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let url = Url::from_file_path(&path).map_err(|_| "Invalid file URL")?;
    let begin = Instant::now();
    let mut doc = ScriptDocument::from_html(
        &html, DocumentConfig {base_url:Some(url.to_string()),viewport:Some(Viewport::new(980,660,1.0,ColorScheme::Light)), ..Default::default()}
    ).without_timer_thread().with_virtual_time();
    doc.execute_scripts();
    pump(&mut doc);
    let mount_ms = millis(begin.elapsed());
    let before = text(&doc);
    if !before.contains("Count: 0") {return Err("React did not mount expected counter".into());}
    if !doc.take_js_errors().is_empty(){return Err("JS exceptions during mount".into());}
    let stylesheets = doc.inner().author_stylesheets().count();
    let mounted_rows = doc.inner().query_selector_all("li").map_err(|e| format!("row selector: {e:?}"))?.len();
    if (mode=="stress" || mode=="stress-memo") && mounted_rows != 200 {
        return Err(format!("stress fixture rendered {mounted_rows} rows rather than 200"));
    }
    if (mode=="astryx"||mode=="stress"||mode=="stress-memo") && stylesheets==0 {
        return Err("no author CSS parsed; invalid styling benchmark".into());
    }
    let click = {
        let d = doc.inner();
        let id = d.query_selector("button").map_err(|e|format!("{e:?}"))?.ok_or("button absent")?;
        DomEvent::new(id,d.get_node(id).ok_or("button node absent")?.synthetic_click_event(Modifiers::empty()))
    };
    let before_click = Instant::now();
    doc.dispatch_dom_event(click);
    pump(&mut doc);
    let click_ms = millis(before_click.elapsed());
    if !text(&doc).contains("Count: 1") || !doc.take_js_errors().is_empty() {
        return Err("React click/state commit failed".into());
    }
    let layout_at = Instant::now();
    doc.inner_mut().resolve(0.0);
    let layout_ms = millis(layout_at.elapsed());
    let input_id = doc.inner().query_selector("input").map_err(|e|format!("{e:?}"))?.ok_or("input absent")?;
    doc.inner_mut().set_focus_to(input_id);
    let before_input = Instant::now();
    doc.handle_ui_event(UiEvent::Ime(BlitzImeEvent::Commit("native hello".into())));
    pump(&mut doc);
    let input_ms = millis(before_input.elapsed());
    if !text(&doc).contains("Echo: native hello") || !doc.take_js_errors().is_empty() {
        return Err("React controlled native input failed".into());
    }
    doc.inner_mut().resolve(0.0);
    const W:u32=980;
    const H:u32=660;
    let start_paint=Instant::now();
    let pixels=render_to_buffer::<VelloCpuImageRenderer,_>(
        |scene| {
            scene.fill(Fill::NonZero,Default::default(),Color::WHITE,Default::default(),
                &Rect::new(0.0,0.0,W as f64,H as f64));
            let mut inner=doc.inner_mut();
            paint_scene(scene,&mut inner,1.0,W,H,0,0);
        }, W,H
    );
    let paint_ms=millis(start_paint.elapsed());
    let dark=pixels.chunks_exact(4).filter(|px|px[0]<240||px[1]<240||px[2]<240).count();
    let nonwhite=dark as f64/(W as f64*H as f64);
    if nonwhite < 0.0001 { return Err(format!("no visible painting: {nonwhite:.6}")); }
    println!("BENCH_SAMPLE mode={mode} mount_ms={mount_ms:.4} click_ms={click_ms:.4} layout_ms={layout_ms:.4} input_ms={input_ms:.4} paint_ms={paint_ms:.4} stylesheets={stylesheets} dom_rows={mounted_rows} nonwhite_ratio={nonwhite:.6} html_bytes={}",
        fs::metadata(&path).map_err(|e|e.to_string())?.len());
    Ok(())
}

fn main() {
    let mut args = env::args().skip(1);
    let mode = args.next().unwrap_or_default();
    let path = args.next().unwrap_or_default();
    if mode.is_empty() || path.is_empty() {
        eprintln!("Usage: native-react-blitz-smoke preact|react|astryx|react-input|astryx-input FILE.html");
        process::exit(2);
    }
    if let Some(bench_mode)=mode.strip_prefix("bench-") {
        if let Err(error)=bench(bench_mode, PathBuf::from(path)) {
            eprintln!("BENCH_FAIL mode={bench_mode}: {error}");
            process::exit(1);
        }
        return;
    }
    let png_path = args.next();
    if let Err(err) = run(&mode, PathBuf::from(path), png_path.as_deref()) {
        eprintln!("NATIVE_REACT_{mode}_FAIL: {err}");
        process::exit(1);
    }
}
