#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    if args.get(1).and_then(|s| s.to_str()) == Some("--desktop-smoke") {
        let binary = args.get(2).map(std::path::PathBuf::from)
            .expect("--desktop-smoke requires pg0 executable path");
        let data = args.get(3).map(std::path::PathBuf::from)
            .expect("--desktop-smoke requires fresh temporary data directory");
        let result = tauri::async_runtime::block_on(project_template_lib::desktop_smoke(binary, data));
        if let Err(err) = result {
            eprintln!("DESKTOP_SMOKE_FAILED: {err}");
            std::process::exit(1);
        }
        return;
    }
    project_template_lib::run();
}
