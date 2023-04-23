#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;

mod file_handlers;
mod hardware;
mod task;
mod test;
mod thread_pool;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn show_splash_window(window: tauri::Window) {
    // 展示启动窗口
    window.get_window("splash").unwrap().show().unwrap();
}

#[tauri::command]
async fn show_main_window(window: tauri::Window) {
    // 隐藏启动窗口
    window.get_window("splash").unwrap().hide().unwrap();
    // 展示主窗口
    let main_window = window.get_window("main").unwrap();
    main_window.show().unwrap();
    main_window.set_focus().unwrap();
}

#[tauri::command]
async fn show_settings_window(window: tauri::Window) {
    // 展示设置窗口
    let setting_window = window.get_window("settings").unwrap();
    setting_window.show().unwrap();
    setting_window.set_focus().unwrap();
}

#[tauri::command]
async fn show_test_window(window: tauri::Window) {
    // 展示测试窗口
    let test_window = window.get_window("test").unwrap();
    test_window.show().unwrap();
    test_window.set_focus().unwrap();
}

#[tauri::command]
async fn get_exe_path(_window: tauri::Window) -> String {
    match std::env::current_exe() {
        Ok(path_buf) => path_buf.to_str().unwrap().to_string(),
        Err(_) => String::from(""),
    }
}

#[tauri::command]
async fn request_window_show(window_label: String, window: tauri::Window) {
    let target_window = window.get_window(&window_label);
    match target_window {
        Some(tw) => tw.emit("request_show", ()).unwrap(),
        None => (),
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
            request_window_show,
            file_handlers::append_file,
            task::add_task,
            test::run_test,
            hardware::system_cpu_info,
            hardware::system_hardware_info,
            thread_pool::set_max_workers_count
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
