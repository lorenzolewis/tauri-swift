#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            tauri_get_file_thumbnail_base64,
            tauri_get_mounts,
            tauri_return_nullable,
            tauri_basic_int
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn tauri_get_file_thumbnail_base64() -> String {
    println!("tauri_get_file_thumbnail_base64 started on the Rust side");
    let path = "/Users";
    let thumbnail = unsafe { get_file_thumbnail_base64(path.into()) };
    std::fs::write("icon.txt", &thumbnail).unwrap();
    println!("Wrote folder icon base64 to icon.txt");

    return "Wrote folder icon base64 to icon.txt".to_string();
}

#[tauri::command]
fn tauri_get_mounts() -> String {
    println!("tauri_get_mounts started on the Rust side");
    let mounts = unsafe { get_mounts() };
    println!("Result: {}", mounts[0].name);

    return mounts[0].name.to_string();
}

#[tauri::command]
fn tauri_return_nullable() -> String {
    println!("tauri_return_nullable started on the Rust side");
    let opt = unsafe { return_nullable(false) };
    println!("function returned data: {}", opt.is_some());

    return opt.is_some().to_string();
}

#[tauri::command]
fn tauri_basic_int() -> String {
    println!("tauri_return_nullable started on the Rust side");
    let opt = unsafe { basic_int() };

    return opt.to_string();
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
    fn basic_int() -> u8;
}
