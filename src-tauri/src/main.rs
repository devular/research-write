// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

#[tauri::command]
fn show_window(app_handle: tauri::AppHandle, label: &str) {
    if let Some(window) = app_handle.get_webview_window(label) {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}

#[tauri::command]
fn hide_window(app_handle: tauri::AppHandle, label: &str) {
    if let Some(window) = app_handle.get_webview_window(label) {
        window.hide().unwrap();
    }
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(desktop)]
            {
                // Fixing the error by using the correct modifier for cross-platform compatibility
                let toggle_shortcut = Shortcut::new(Some(Modifiers::META | Modifiers::SHIFT), Code::KeyH);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new().with_handler(move |app, shortcut, event| {
                        if shortcut == &toggle_shortcut {
                            match event.state() {
                                ShortcutState::Pressed => {
                                    let essay_window = app.get_webview_window("essay-app").unwrap();
                                    let perplexity_window = app.get_webview_window("perplexity-ai").unwrap();

                                    if essay_window.is_visible().unwrap() {
                                        essay_window.hide().unwrap();
                                        perplexity_window.show().unwrap();
                                        perplexity_window.set_focus().unwrap();
                                    } else {
                                        perplexity_window.hide().unwrap();
                                        essay_window.show().unwrap();
                                        essay_window.set_focus().unwrap();
                                    }
                                }
                                _ => {}
                            }
                        }
                    })
                    .build(),
                )?;

                app.global_shortcut().register(toggle_shortcut)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![show_window, hide_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
