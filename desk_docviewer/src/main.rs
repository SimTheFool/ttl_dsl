// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::collections::HashMap;
use std::fs;

use docviewer::constants::{ASSETS as VIEWS_PATH, VIEWER_WINDOW};
use docviewer::ConfigFromFile;
use lib_core::prefab::FSResolver;
use lib_core::{commands::AssembleFromPath, prefab::JsonFormatter};
use tauri::{LogicalSize, Manager, Size, State, Window};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let viewer_win =
                tauri::WindowBuilder::new(app, VIEWER_WINDOW, tauri::WindowUrl::App("/".into()))
                    .build()?;

            viewer_win.set_size(Size::Logical(LogicalSize {
                height: 980.0,
                width: 700.0,
            }))?;
            viewer_win.set_resizable(true)?;
            viewer_win.set_title(VIEWER_WINDOW)?;

            app.manage(viewer_win);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_templates,
            get_template_data,
            print
        ])
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
fn get_template_data(
    data_file: &str,
    resolution_dir: &str,
) -> Result<(serde_json::Value, HashMap<String, String>), String> {
    let resolver = FSResolver::new(resolution_dir);
    let config = ConfigFromFile::new(resolution_dir).map_err(|e| e.to_string())?;
    let formatter = JsonFormatter {};

    let assemble_from_path = AssembleFromPath::new(&resolver, &config, &formatter);
    let json = assemble_from_path
        .execute(data_file)
        .map_err(|e| e.to_string())?;

    let path = std::path::Path::new(data_file);

    let images = fs::read_dir(path.parent().ok_or("Invalid path")?)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            let path = match path.extension() {
                Some(ext) if ext == "png" => Some(path),
                Some(ext) if ext == "jpg" => Some(path),
                Some(ext) if ext == "jpeg" => Some(path),
                _ => None,
            };

            path.map(|p| {
                let name = p.file_stem().unwrap().to_str().unwrap().to_string();
                let path = p.to_str().unwrap().to_string();
                (name, path)
            })
        })
        .collect::<HashMap<String, String>>();

    Ok((json, images))
}

#[tauri::command]
fn print(window: State<Window>) -> Result<(), String> {
    window.print().map_err(|e| e.to_string())
}
