// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use docviewer::constants::{ASSETS as VIEWS_PATH, VIEWER_WINDOW};
use tauri::{LogicalSize, Size};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let viewer_win =
                tauri::WindowBuilder::new(app, VIEWER_WINDOW, tauri::WindowUrl::App("/".into()))
                    .build()?;

            viewer_win.set_size(Size::Logical(LogicalSize {
                height: 600.0,
                width: 800.0,
            }))?;
            viewer_win.set_resizable(true)?;
            viewer_win.set_title(VIEWER_WINDOW)?;

            let _viewer_win = viewer_win.clone();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_templates])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_templates() -> Result<Vec<String>, &'static str> {
    VIEWS_PATH
        .iter()
        .map(|p| p.get_path())
        .collect::<Result<Vec<String>, &'static str>>()
}

fn get_json_data(datafile: &str, resolutionDir: &str) -> Result<serde_json::Value, String> {
    todo!()
}
