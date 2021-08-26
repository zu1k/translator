#![windows_subsystem = "windows"]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(clippy::all, rust_2018_idioms)]

use online_api as deepl;
use std::sync::mpsc;
use std::thread;
use tauri_hotkey::HotkeyManager;
use copy_translator::register_hotkey;

fn main() {
    let mut hk_mng = HotkeyManager::new();
    let (tx_hk, rx) = mpsc::channel();
    register_hotkey(&mut hk_mng, tx_hk.clone());

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
                let _ = hk_mng.unregister_all();
                let app = copy_translator::MyApp::new(text, rx, task_tx);
                let app = Box::new(app);
                let native_options = eframe::NativeOptions {
                    always_on_top: true,
                    decorated: false,
                    initial_window_size: Some(egui::vec2(500.0, 196.0)),
                    ..Default::default()
                };
                eframe::run_native_return(app, native_options);
                register_hotkey(&mut hk_mng, tx_hk.clone());
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
    }
}
