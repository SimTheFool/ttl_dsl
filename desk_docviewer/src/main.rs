// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{LogicalSize, Size};

const VIEWER_WINDOW: &str = "Viewer";

//#[iftree::include_file_tree("paths = '/src/app/*/page.tsx'")]
//#[derive(Debug)]
//pub struct HtmlView {
//    relative_path: &'static str,
//}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let viewer_win = tauri::WindowBuilder::new(
                app,
                VIEWER_WINDOW,
                tauri::WindowUrl::App("/SRDocument".into()),
            )
            .build()?;

            viewer_win.set_size(Size::Logical(LogicalSize {
                height: 600.0,
                width: 800.0,
            }))?;
            viewer_win.set_resizable(true)?;
            viewer_win.set_title(VIEWER_WINDOW)?;

            //println!("ASSETS: {:#?}", ASSETS);

            //assert_eq!(ASSETS.len(), 3);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
