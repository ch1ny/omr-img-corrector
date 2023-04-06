use std::{env, fs, path::PathBuf};

fn main() {
    tauri_build::build();

    // 获取 debug/release mode
    let profile = env::var("PROFILE").unwrap();
    // 复制 mydll.dll
    let src_path = PathBuf::from("../assets/opencv_world460.dll");
    let dest_path = PathBuf::from("./target")
        .join(profile)
        .join("opencv_world460.dll");

    fs::copy(&src_path, &dest_path).unwrap();
}
