pub mod command;

use tauri::{
    Manager,
    menu::{Menu, MenuItem},
    tray::{MouseButton, TrayIconBuilder},
};

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
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click { button, .. } = event {
                        if button == MouseButton::Left {
                            let app = tray.app_handle();
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.unminimize();
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            open_command_palette,
            close_command_palette,
            command::click::click,
            command::fetch_bookmark_list::fetch_bookmark_list,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
