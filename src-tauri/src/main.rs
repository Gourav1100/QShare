// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;

use if_addrs::IfAddr;
// Dependencies
use modules::events::handle_event;
use modules::handler::generic_handler;
use modules::menu::init_menu;
use modules::socket_handler::socket_handler;
use std::thread;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            setup(app.get_window("main").unwrap());
            Ok(())
        })
        .menu(init_menu())
        .on_menu_event(|event| handle_event(&event))
        .invoke_handler(tauri::generate_handler![generic_handler])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn setup(window: tauri::Window) {
    let _port = "8080";
    if let Ok(if_addrs) = if_addrs::get_if_addrs() {
        for if_addr in if_addrs {
            if let IfAddr::V4(addr) = if_addr.addr {
                let ip_addr = addr.ip;
                println!(
                    "Interface: {}, IP address: {}, running at {}:{}",
                    if_addr.name, ip_addr, ip_addr, _port
                );
                let window_clone = window.clone();
                thread::spawn(move || {
                    socket_handler(ip_addr.to_string() + ":" + _port, window_clone)
                });
            }
        }
    }
}
