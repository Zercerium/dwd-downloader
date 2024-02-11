// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use build_info::build_infos;
use dwd_command::{async_test, dwd_filename_suggestion, dwd_request};
use specta::export;

mod build_info;
mod dwd_command;

fn main() {
    #[cfg(debug_assertions)]
    export_types();

    tauri::Builder::default()
        .setup(|app| {
            app.handle()
                .plugin(tauri_plugin_updater::Builder::new().build())?;
            Ok(())
        })
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            build_infos,
            dwd_request,
            dwd_filename_suggestion,
            async_test,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(debug_assertions)]
fn export_types() {
    export::ts("../utils/bindings.ts").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_types() {
        export_types();
    }
}
