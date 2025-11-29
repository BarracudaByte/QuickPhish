//use tauri::ipc::Response;
use std::sync::{Arc, Mutex};
use tauri::menu::{CheckMenuItemBuilder, MenuBuilder, SubmenuBuilder, PredefinedMenuItem};
use tauri::{Emitter, Manager};
use tauri_plugin_log::{Target, TargetKind};
use tauri_plugin_store::StoreExt;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

mod analysis_commands;
mod header_verification;
mod indicators;
mod parsed_eml;
mod risk_data;
mod store_commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Shortcuts
            #[cfg(desktop)]
            {
                let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::META), Code::KeyV);
                let ctrl_o_shortcut = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::META), Code::KeyO);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new().with_handler(move |app_handle, shortcut, event| {
                        println!("Shortcut {:?}", shortcut);
                        if shortcut == &ctrl_n_shortcut {
                            match event.state() {
                              ShortcutState::Pressed => {
                                println!("Ctrl-V Pressed!");
                              }
                              ShortcutState::Released => {
                                println!("Ctrl-V Released!");
                              }
                            }
                        } else if shortcut == &ctrl_o_shortcut {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    // only show on release
                                }
                                ShortcutState::Released => {
                                    app_handle.emit("open-file", {}).unwrap();
                                }
                            }
                        }
                    })
                    .build(),
                )?;

                app.global_shortcut().register(ctrl_o_shortcut)?;
            }

            // Create the store instance
            let store = app.store("app_data.json")?;

            // Wrap the store in Arc<Mutex> for shared, mutable access
            let shared_store = Arc::new(Mutex::new(store));

            // Manage the shared store as Tauri state
            app.manage(shared_store);

            // Window Menu
            /*let check_dark_theme = CheckMenuItemBuilder::with_id("dark_theme", "Dark Theme")
            .checked(false)
            .build(app)?;*/

            let app_menu = SubmenuBuilder::new(app, "App")
                //.item(&check_dark_theme)
                .text("toggle_theme", "Toggle Theme")
                .separator()
                .text("quit", "Quit")
                .build()?;

            let file_menu = SubmenuBuilder::new(app, "File")
                .text("open", "Open")
                .build()?;

            let edit_menu = SubmenuBuilder::new(app, "Edit")
                .copy()
                .paste()
                .build()?;

            let menu = MenuBuilder::new(app)
                .items(&[&app_menu, &file_menu, &edit_menu])
                .build()?;

            app.set_menu(menu)?;

            app.on_menu_event(move |app_handle: &tauri::AppHandle, event| {
                println!("menu event: {:?}", event.id());

                match event.id().0.as_str() {
                    "toggle_theme" => {
                        //let dark_theme = check_dark_theme.is_checked().unwrap_or(true);
                        //let _ = check_dark_theme.set_checked(dark_theme);
                        app_handle.emit("toggle-theme", true).unwrap();
                    }
                    "quit" => {
                        app_handle.exit(0);
                    }
                    "open" => {
                        app_handle.emit("open-file", {}).unwrap();
                    }
                    _ => {
                        println!("unexpected menu event");
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            analysis_commands::load_eml,
            store_commands::get_summary_template,
            store_commands::update_summary_template,
            store_commands::get_lists,
            store_commands::update_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
