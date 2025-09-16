use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub async fn set_value(
    app: AppHandle,
    store: String,
    key: String,
    value: serde_json::Value,
) -> Result<(), String> {
    let store = app.store(&store).map_err(|e| e.to_string())?;
    store.set(key, value);
    store.save().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_value(
    app: AppHandle,

    store: String,
    key: String,
) -> Result<Option<serde_json::Value>, String> {
    let store = app.store(&store).map_err(|e| e.to_string())?;
    let value = store.get(&key);
    Ok(value)
}
