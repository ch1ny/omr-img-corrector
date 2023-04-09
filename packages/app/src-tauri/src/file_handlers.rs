use std::{fs::OpenOptions, io::prelude::Write, path::Path};

#[tauri::command]
pub fn append_file(target_file_path: String, string_data: String) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(Path::new(&target_file_path))
        .unwrap();

    file.write_all(string_data.as_bytes()).unwrap();
}
