//! Runtime smoke test: genuine React/Preact JS execution in Boa/Blitz.
//! Does not validate graphics, accessibility, CSS, or packaging.
use blitz_dom::{Document, DocumentConfig};
use blitz_traits::events::{DomEvent, UiEvent, BlitzImeEvent};
use blitz_vibey_script::ScriptDocument;
use keyboard_types::Modifiers;
use std::{env,fs,path::PathBuf,process};
use url::Url;

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
fn run(mode: &str, path: PathBuf) -> Result<(), String> {
    let path = path.canonicalize().map_err(|e| format!("No HTML {}: {e}", path.display()))?;
    let html = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let url = Url::from_file_path(&path).map_err(|_| "Invalid local file path")?;
    let mut doc = ScriptDocument::from_html(
        &html,
        DocumentConfig { base_url: Some(url.to_string()), ..Default::default() },
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
       "astryx" | "astryx-input" | "react-input" => "Count: 0",
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
    println!("NATIVE_REACT_{mode}_PASS");
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
    if let Err(err) = run(&mode, PathBuf::from(path)) {
        eprintln!("NATIVE_REACT_{mode}_FAIL: {err}");
        process::exit(1);
    }
}
