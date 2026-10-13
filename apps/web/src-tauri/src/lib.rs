pub mod desktop;

use desktop::{CreateNote, DesktopDatabase, Note};
use tauri::{Manager, RunEvent, State};
use std::path::PathBuf;

#[tauri::command]
async fn list_notes(database: State<'_, DesktopDatabase>) -> Result<Vec<Note>, String> {
    database.list().await
}

#[tauri::command]
async fn create_note(input: CreateNote, database: State<'_, DesktopDatabase>) -> Result<Note, String> {
    database.create(input).await
}

pub fn run() {
    let app = tauri::Builder::default()
        // Close the dedicated PostgreSQL instance before the WebView is
        // destroyed. Exit remains a fallback for non-window exit paths.
        .on_window_event(|window, event| {
            if matches!(event, tauri::WindowEvent::CloseRequested { .. }) {
                let database = window.state::<DesktopDatabase>();
                if let Err(err) = tauri::async_runtime::block_on(database.shutdown()) {
                    eprintln!("Desktop database close-request shutdown failed: {err}");
                }
            }
        })
        .plugin(tauri_plugin_single_instance::init(|app, _, _| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.set_focus();
            }
        }))
        .setup(|app| {
            let resource = app.path().resource_dir()?;
            let binary = resource.join("binaries").join(if cfg!(windows) { "pg0.exe" } else { "pg0" });
            if !binary.is_file() {
                return Err(format!("Missing bundled database runtime: {}", binary.display()).into());
            }
            let app_data = app.path().app_local_data_dir()?;
            app.manage(DesktopDatabase::for_app(binary, app_data));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![list_notes, create_note])
        .build(tauri::generate_context!())
        .expect("could not build Tauri app");

    app.run(|app, event| {
        if let RunEvent::Exit = event {
            let database = app.state::<DesktopDatabase>();
            if let Err(err) = tauri::async_runtime::block_on(database.shutdown()) {
                eprintln!("Could not gracefully close desktop database: {err}");
            }
        }
    });
}

pub async fn desktop_smoke(binary: PathBuf, app_data: PathBuf) -> Result<(), String> {
    let id = format!("template-smoke-{}", uuid::Uuid::new_v4().simple());
    // Isolate all test data under a uniquely named child directory. Even a caller
    // passing the production app data path cannot make the test drop live data.
    let app_data = app_data.join(&id);
    let first = DesktopDatabase::new(binary.clone(), app_data.clone(), id.clone());
    let note = first.create(CreateNote { title: "survives restart".into(), body: "native desktop".into() }).await?;
    if !first.list().await?.iter().any(|n| n.id == note.id) {
        return Err("Insert was not readable".into());
    }
    first.shutdown().await?;
    let second = DesktopDatabase::new(binary.clone(), app_data.clone(), id.clone());
    if !second.list().await?.iter().any(|n| n.id == note.id) {
        return Err("Desktop note did not survive a full PostgreSQL restart".into());
    }
    second.shutdown().await?;
    // Simulate a crash: the Rust owner vanishes without calling shutdown.
    let crashed = DesktopDatabase::new(binary.clone(), app_data.clone(), id.clone());
    if !crashed.list().await?.iter().any(|n| n.id == note.id) {
        return Err("Crash-simulation database was unreadable".into());
    }
    drop(crashed);
    // On relaunch, the dedicated pg0 process is already alive. The new owner
    // must reattach and take responsibility for graceful shutdown.
    let recovered = DesktopDatabase::new(binary.clone(), app_data.clone(), id.clone());
    if !recovered.list().await?.iter().any(|n| n.id == note.id) {
        return Err("Database could not be recovered after owner crash".into());
    }
    recovered.shutdown().await?;
    let output = std::process::Command::new(&binary)
        .args(["info", "--name", &id, "-o", "json"])
        .output().map_err(|e| e.to_string())?;
    let state: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| e.to_string())?;
    if state["running"] == true {
        return Err("Recovered desktop instance was not stopped on exit".into());
    }
    // Test-only instance; remove metadata and files in this disposable location.
    let status = std::process::Command::new(&binary).args(["drop", "--name", &id, "--force"])
        .status().map_err(|e| e.to_string())?;
    if !status.success() { return Err("Could not clean up smoke-test cluster".into()); }
    println!("DESKTOP_PERSISTENCE_RESTART_OK");
    Ok(())
}
