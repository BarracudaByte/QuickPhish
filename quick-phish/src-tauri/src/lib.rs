//use tauri::ipc::Response;
use tauri::Manager; // State
use tauri_plugin_store::StoreExt;
use std::sync::{Arc, Mutex}; 

mod analysis_commands;
mod template_commands;
mod indicators;
mod parsed_eml;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Create the store instance
            let store = app.store("app_data.json")?;
            
            // Wrap the store in Arc<Mutex> for shared, mutable access
            let shared_store = Arc::new(Mutex::new(store));

            // Manage the shared store as Tauri state
            app.manage(shared_store);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![analysis_commands::load_eml, template_commands::get_summary_template, template_commands::update_summary_template])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
