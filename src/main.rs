#![windows_subsystem = "windows"]


#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]


use tauri_hotkey::{HotkeyManager, parse_hotkey};
use enigo::{Enigo, Key, KeyboardControllable};
use cli_clipboard;
use std::sync::mpsc;
use online_api::*;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use core::panic;

    let (tx, rx) = mpsc::channel();

    let mut hk_mng = HotkeyManager::new();
    let mut enigo = Enigo::new();


    if let Err(err) =  hk_mng.register(parse_hotkey("CTRL+SHIFT+D").unwrap(), move || {
        std::process::exit(0)
    }) {
        panic!("{}", err)
    }

    if let Err(err) =  hk_mng.register(parse_hotkey("CTRL+D").unwrap(), move || {
        enigo.key_down(Key::Control);
        enigo.key_click(Key::Layout('c'));
        enigo.key_up(Key::Control);
        if let Ok(text) = cli_clipboard::get_contents() {
            if let Err(err) = tx.send(text) {
                panic!("{}", err)
            }
        }        
    }) {
        panic!("{}", err)
    }

    loop {
        match rx.recv() {
            Ok(text) => {
                println!("{}", text);
                
                match translate(text) {
                    Ok(text) => {
                        let app = copy_translator::MyApp::new(text);
                        let native_options = eframe::NativeOptions {
                            always_on_top: true,
                            decorated: false,
                            initial_window_size: Some(egui::vec2(500.0, 100.0)),
                            ..Default::default()
                        };
                        eframe::run_native_return(Box::new(app), native_options);
                    },
                    Err(err) => {
                        panic!("{}", err)
                    }
                }
            },
            Err(err) => {
                panic!("{}", err)
            }
        }
    }
}
