// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use docviewer::constants::{ASSETS as VIEWS_PATH, VIEWER_WINDOW};
use tauri::{CustomMenuItem, LogicalSize, Menu, Size};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let default_view_path = VIEWS_PATH[0].get_path().map_err(|e| anyhow::anyhow!(e))?;

            let menu = Menu::new().add_item(CustomMenuItem::new("print", "Print"));
            let viewer_win = tauri::WindowBuilder::new(
                app,
                VIEWER_WINDOW,
                tauri::WindowUrl::App(default_view_path.into()),
            )
            .menu(menu)
            .build()?;

            viewer_win.set_size(Size::Logical(LogicalSize {
                height: 600.0,
                width: 800.0,
            }))?;
            viewer_win.set_resizable(true)?;
            viewer_win.set_title(VIEWER_WINDOW)?;

            let _viewer_win = viewer_win.clone();
            viewer_win.on_menu_event(move |event| {
                if event.menu_item_id() == "print" {
                    _viewer_win.print().unwrap();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
