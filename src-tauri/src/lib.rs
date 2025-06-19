use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_command_palette(app: tauri::AppHandle) {
    let _ = tauri::WebviewWindowBuilder::new(
        &app,
        "command-palette",
        tauri::WebviewUrl::App("/command-palette".into()),
    )
    .title("Command Palette")
    .inner_size(800.0, 400.0)
    .decorations(false)
    .resizable(false)
    .always_on_top(true)
    .focused(true)
    .skip_taskbar(true)
    .shadow(false)
    .center()
    .transparent(true)
    .visible(true)
    .build()
    .map(|command_palette_window| {
        let _ = command_palette_window.set_focus();
    });
}

#[tauri::command]
async fn close_command_palette(app: tauri::AppHandle) -> bool {
    if let Some(window) = app.get_webview_window("command-palette") {
        let result = window.close();
        result.is_ok()
    } else {
        false
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            open_command_palette,
            close_command_palette
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
