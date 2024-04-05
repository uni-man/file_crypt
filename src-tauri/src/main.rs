// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{window::Window, LogicalSize, Manager, Size};

mod file_crypt;

// Learn more about Tauri commands at
// https://tauri.app/v1/guides/features/command

#[tauri::command(rename_all = "snake_case")]
fn encrypt(filepath: &str, secret_key: &str) -> String {
    if let Err(error) = file_crypt::encrypt(filepath,secret_key) { error }
    else {
        format!("Your file has been successfully encrypted!")
    }
}

#[tauri::command(rename_all = "snake_case")]
fn decrypt(filepath: &str, secret_key: &str) -> String {
    if let Err(error) = file_crypt::decrypt(filepath, secret_key) { error }
    else {
        format!("Your file has been successfully decrypted!")
    }
}

fn main() {
    tauri::Builder::default()
    .setup(|app| {
        let w = app.windows();
        let w2 = w.get("main").unwrap();
        let x = w2.inner_size().unwrap();
        let height = x.height;
        Window::set_size(w2, Size::Logical(LogicalSize {width: 490.0, height: height as f64 * 0.75})).unwrap();
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![encrypt, decrypt])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
