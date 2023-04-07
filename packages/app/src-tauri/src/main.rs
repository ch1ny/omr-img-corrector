#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod hardware;
mod test;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn show_splash_window(window: tauri::Window) {
    // 展示启动窗口
    window.get_window("splash").unwrap().show().unwrap();
}

#[tauri::command]
async fn show_main_window(window: tauri::Window) {
    // 关闭启动窗口
    window.get_window("splash").unwrap().close().unwrap();
    // 展示主窗口
    window.get_window("main").unwrap().show().unwrap();
}

#[tauri::command]
async fn show_settings_window(window: tauri::Window) {
    // 展示设置窗口
    window.get_window("settings").unwrap().show().unwrap();
}

#[tauri::command]
async fn show_test_window(window: tauri::Window) {
    // 展示测试窗口
    window.get_window("test").unwrap().show().unwrap();
}

#[tauri::command]
async fn get_exe_path(_window: tauri::Window) -> String {
    match std::env::current_exe() {
        Ok(path_buf) => return path_buf.to_str().unwrap().to_string(),
        Err(_) => String::from(""),
    }
}

#[tauri::command]
async fn exit_app() {
    // 退出程序
    std::process::exit(1);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            show_splash_window,
            show_main_window,
            show_settings_window,
            show_test_window,
            get_exe_path,
            exit_app,
            test::run_test,
            hardware::system_cpu_info,
            hardware::system_hardware_info,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
