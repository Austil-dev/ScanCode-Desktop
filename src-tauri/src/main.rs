// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    scancode_desktop_lib::run() // Appel de la fonction run() depuis la librairie "lib.rs" pour démarrer l'application Tauri.
}
