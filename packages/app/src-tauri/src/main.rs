#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod hardware;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn show_main_window(window: tauri::Window) {
    // 展示主窗口
    window.get_window("main").unwrap().show().unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            show_main_window,
            hardware::system_cpu_info,
            hardware::system_hardware_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
