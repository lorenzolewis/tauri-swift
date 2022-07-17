#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![tauri_get_mounts])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use swift_rs::{SRObjectArray, SRString};

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

#[tauri::command]
fn tauri_get_mounts() -> String {
    let mounts = unsafe { return_nullable(true) };
    return mounts.to_string();
}

extern "C" {
    // fn get_mounts() -> SRObjectArray<Volume>;
    fn return_nullable(null: bool) -> bool;
}
