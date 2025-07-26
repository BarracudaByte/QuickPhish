use serde_json::json;
use tauri::{AppHandle, State};
use tauri_plugin_store::{StoreBuilder, Store, StoreExt};

const SUMARY_TEMPLATE: &'static str = "summary_template";

#[tauri::command]
pub fn get_summary_template(app_handle: AppHandle) -> serde_json::Value {
    let store = app_handle.store("app_data.json");

    let default_template = json!({ "template": "NA" });
    if store.is_ok() {
        return store.unwrap().get(SUMARY_TEMPLATE).unwrap_or(default_template);
    }

    return default_template;
}

#[tauri::command]
pub fn update_summary_template(summary: &str, app_handle: AppHandle) -> bool {
    let store = app_handle.store("app_data.json");
    let template = json!({ "template": summary.to_string() });

    if store.is_ok() {
        store.unwrap().set(SUMARY_TEMPLATE, template);
        return true
    }
    return false;
}