#![windows_subsystem = "windows"]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, rust_2018_idioms)]

use cli_clipboard;
use enigo::{Enigo, Key, KeyboardControllable};
use online_api as deepl;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;
use tauri_hotkey::{parse_hotkey, HotkeyManager};

fn main() {
    let mut hk_mng = HotkeyManager::new();
    let (tx, rx) = mpsc::channel();
    register_hotkey(&mut hk_mng, tx);

    loop {
        match rx.recv() {
            Ok(text) => {
                println!("{}", text);
                let (tx, rx) = mpsc::sync_channel(1);
                let (task_tx, task_rx) = mpsc::sync_channel(1);
                thread::spawn(move || {
                    while let Ok((text, target_lang, source_lang)) = task_rx.recv() {
                        let _ = match deepl::translate(text, target_lang, source_lang) {
                            Ok(text) => tx.send(text),
                            Err(err) => tx.send(err.to_string()),
                        };
                    }
                });
                let app = copy_translator::MyApp::new(text, rx, task_tx);
                let app = Box::new(app);
                let native_options = eframe::NativeOptions {
                    always_on_top: true,
                    decorated: false,
                    initial_window_size: Some(egui::vec2(500.0, 196.0)),
                    ..Default::default()
                };
                eframe::run_native_return(app, native_options);
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
    }
}

fn register_hotkey(hk_mng: &mut HotkeyManager, tx: Sender<String>) {
    // CTRL+SHIFT+D quit
    if let Err(err) = hk_mng.register(parse_hotkey("CTRL+SHIFT+D").unwrap(), move || {
        std::process::exit(0)
    }) {
        panic!("{}", err)
    }

    // CTRL+D launch
    let tx_d = tx.clone();
    if let Err(err) = hk_mng.register(parse_hotkey("CTRL+D").unwrap(), move || {
        let mut enigo = Enigo::new();
        enigo.key_down(Key::Control);
        enigo.key_click(Key::Layout('c'));
        enigo.key_up(Key::Control);
        thread::sleep(Duration::from_millis(100));
        if let Ok(text) = cli_clipboard::get_contents() {
            if let Err(err) = tx_d.send(text) {
                panic!("{}", err)
            }
        }
    }) {
        panic!("{}", err)
    }

    // CTRL+Q launch
    let tx_q = tx.clone();
    if let Err(err) = hk_mng.register(parse_hotkey("CTRL+Q").unwrap(), move || {
        let mut enigo = Enigo::new();
        enigo.key_down(Key::Control);
        enigo.key_click(Key::Layout('c'));
        enigo.key_up(Key::Control);
        thread::sleep(Duration::from_millis(100));
        if let Ok(text) = cli_clipboard::get_contents() {
            if let Err(err) = tx_q.send(text) {
                panic!("{}", err)
            }
        }
    }) {
        panic!("{}", err)
    }
}
