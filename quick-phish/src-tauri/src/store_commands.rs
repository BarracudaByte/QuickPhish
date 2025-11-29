use minijinja::{context, Environment};
use serde_json::json;
use tauri::AppHandle; // State
use tauri_plugin_store::StoreExt; // StoreBuilder, Store,

pub const SUMMARY_TEMPLATE: &'static str = "summary_template";
pub const WHITELIST: &'static str = "whitelist";
pub const BLACKLIST: &'static str = "blacklist";
pub const STORE_NAME: &'static str = "app_data.json";

pub fn get_template(name: &str, app_handle: &AppHandle) -> serde_json::Value {
    let store = app_handle.store("app_data.json");

    let default_template = json!({ "template": "NA" });
    if store.is_ok() {
        return store.unwrap().get(name).unwrap_or(default_template);
    }

    return default_template;
}

#[tauri::command]
pub fn get_lists(app_handle: AppHandle) -> serde_json::Value {
    let store = app_handle.store("app_data.json");
    let default_list = json!({ "content": "" });

    if let Ok(store) = store {
        let whitelist = store.get(WHITELIST).unwrap_or(default_list.clone());
        let blacklist = store.get(BLACKLIST).unwrap_or(default_list.clone());
        return json!({
            WHITELIST: whitelist,
            BLACKLIST: blacklist
        });
    }
    return json!({
        WHITELIST: default_list.clone(),
        BLACKLIST: default_list
    });
}

#[tauri::command]
pub fn update_list(list: &str, content: &str, app_handle: AppHandle) -> bool {
    if list != WHITELIST && list != BLACKLIST {
        // basicaly unexpected key... should not happen
        panic!("Invalid list name: {}", list);
    }
    let store = app_handle.store(STORE_NAME);
    let data = json!({ "content": content.to_string() });
    if store.is_ok() {
        store.unwrap().set(list, data);
        return true;
    }
    return false;
}

#[tauri::command]
pub fn get_summary_template(app_handle: AppHandle) -> serde_json::Value {
    return get_template(SUMMARY_TEMPLATE, &app_handle);
}

#[tauri::command]
pub fn update_summary_template(summary: &str, app_handle: AppHandle) -> bool {
    let mut env = Environment::new();
    let valid_template = env.add_template(SUMMARY_TEMPLATE, summary);
    if valid_template.is_err() {
        return false;
    }

    let store = app_handle.store(STORE_NAME);
    let template = json!({ "template": summary.to_string() });

    if store.is_ok() {
        store.unwrap().set(SUMMARY_TEMPLATE, template);
        return true;
    }
    return false;
}
