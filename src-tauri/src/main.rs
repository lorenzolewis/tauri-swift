#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            tauri_get_file_thumbnail_base64,
            tauri_get_mounts,
            tauri_return_nullable
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn tauri_get_file_thumbnail_base64() {
    let path = "/Users";
    let thumbnail = unsafe { get_file_thumbnail_base64(path.into()) };
    std::fs::write("icon.txt", &thumbnail).unwrap();
    println!("Wrote folder icon base64 to icon.txt");

    let mounts = unsafe { get_mounts() };
    println!("First Volume Name: {}", mounts[0].name);
}

#[tauri::command]
fn tauri_get_mounts() {
    let opt = unsafe { return_nullable(true) };
    println!("function returned nil: {}", opt.is_none());
}

#[tauri::command]
fn tauri_return_nullable() {
    let opt = unsafe { return_nullable(false) };
    println!("function returned data: {}", opt.is_some());
}

use swift_rs::{SRArray, SRObject, SRObjectArray, SRString};

#[repr(C)]
struct Volume {
    pub name: SRString,
    path: SRString,
    total_capacity: usize,
    available_capacity: usize,
    is_removable: bool,
    is_ejectable: bool,
    is_root_filesystem: bool,
}

#[repr(C)]
struct Test {
    pub null: bool,
}

extern "C" {
    fn get_file_thumbnail_base64(path: SRString) -> SRString;
    fn get_mounts() -> SRObjectArray<Volume>;
    fn return_nullable(null: bool) -> Option<SRObject<Test>>;
}
