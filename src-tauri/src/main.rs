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
        let windows = app.windows();
        let result = windows.get("main");
        if let None = result {
            println!("Main window not found");
            return Ok(());
        }

        let window = result.unwrap();
        let result = window.inner_size();
        if let Err(error) = result {
            println!("{:?}", error);
            return Ok(());
        }

        let height = result.unwrap().height;
        let _ = Window::set_size(window, Size::Logical(LogicalSize {width: 490.0, height: 780.0}));
        let _ = Window::set_max_size(window, Some(Size::Logical(LogicalSize { width: 490.0, height: height as f64 * 0.75 })));

        Ok(())
    })
    .invoke_handler(tauri::generate_handler![encrypt, decrypt])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
