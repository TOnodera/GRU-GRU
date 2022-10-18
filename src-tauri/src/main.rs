#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{thread, time::Duration};

use mouse_rs::Mouse;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn grugru() {
    let mouse = Mouse::new();
    let mut i = 0.0;
    loop {
        thread::sleep(Duration::from_millis(100));
        let r = 50.0;
        let x = f64::cos(i) * r + 500.0;
        let y = f64::sin(i) * r + 500.0;
        println!("x: {}, y: {}, i: {}", x, y, i);
        mouse
            .move_to(x as i32, y as i32)
            .expect("Unable to move mouse");
        i += 1.0;
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![grugru])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
