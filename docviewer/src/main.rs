// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use ::futures::future;
use docviewer::constants::{ASSETS as VIEWS_PATH, VIEWER_WINDOW};
use docviewer::ConfigFromFile;
use lib_core::prefab::FSResolver;
use lib_core::{commands::AssembleFromPath, prefab::JsonFormatter};
use std::collections::HashMap;
use std::fs::{self};
use tauri::{LogicalSize, Manager, Size, State, Window};
use tokio::task::spawn_blocking;
// aabbccssaaaaaa
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let viewer_win =
                tauri::WindowBuilder::new(app, VIEWER_WINDOW, tauri::WindowUrl::App("/".into()))
                    .build()?;

            viewer_win.set_size(Size::Logical(LogicalSize {
                height: 700.0,
                width: 720.0,
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
async fn get_templates() -> Result<Vec<String>, &'static str> {
    VIEWS_PATH
        .iter()
        .map(|p| p.get_path())
        .collect::<Result<Vec<String>, &'static str>>()
}

#[tauri::command]
async fn get_template_data(
    data_file: &str,
    resolution_dir: &str,
) -> Result<(serde_json::Value, HashMap<String, String>), String> {
    let resolver = FSResolver::new(resolution_dir);
    let config = ConfigFromFile::new(resolution_dir).map_err(|e| e.to_string())?;
    let formatter = JsonFormatter {};

    let data_file_for_json = data_file.to_string();
    let handle_json = spawn_blocking(move || {
        let assemble_from_path = AssembleFromPath::new(&resolver, &config, &formatter);
        assemble_from_path
            .execute(&data_file_for_json)
            .map_err(|e| e.to_string())
    });

    let data_file_for_images = data_file.to_string();
    let handle_images_paths = spawn_blocking(move || -> Result<HashMap<String, String>, String> {
        let path = std::path::Path::new(&data_file_for_images);
        let images = fs::read_dir(path.parent().ok_or("Invalid path".to_string())?)
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

        Ok(images)
    });

    let (json, images) = future::try_join(handle_json, handle_images_paths)
        .await
        .map_err(|e| e.to_string())?;

    Ok((json?, images?))
}

#[tauri::command]
fn print(window: State<Window>) -> Result<(), String> {
    window.print().map_err(|e| e.to_string())
}
