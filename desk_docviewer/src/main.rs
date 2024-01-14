// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use docviewer::constants::{ASSETS as VIEWS_PATH, VIEWER_WINDOW};
use docviewer::ConfigFromFile;
use lib_core::prefab::FSResolver;
use lib_core::{commands::AssembleFromPath, prefab::JsonFormatter};
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
        .invoke_handler(tauri::generate_handler![get_templates, get_json_data])
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

#[tauri::command]
fn get_json_data(data_file: &str, resolution_dir: &str) -> Result<serde_json::Value, String> {
    let resolver = FSResolver::new(resolution_dir);
    let config = ConfigFromFile::new(resolution_dir).map_err(|e| e.to_string())?;
    let formatter = JsonFormatter {};

    let assemble_from_path = AssembleFromPath::new(&resolver, &config, &formatter);
    let json = assemble_from_path
        .execute(data_file)
        .map_err(|e| e.to_string())?;

    Ok(json)
}
