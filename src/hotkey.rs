use enigo::{Enigo, Key, KeyboardControllable};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use tauri_hotkey::{parse_hotkey, HotkeyManager};

pub fn register_hotkey(hk_mng: &mut HotkeyManager, tx: Sender<String>) {
    // CTRL+SHIFT+D quit
    if let Err(err) = hk_mng.register(parse_hotkey("CTRL+SHIFT+D").unwrap(), move || {
        std::process::exit(0)
    }) {
        panic!("{:?}", err)
    }

    // CTRL+D launch
    let tx_d = tx.clone();
    if let Err(err) = hk_mng.register(parse_hotkey("CTRL+D").unwrap(), move || {
        if let Some(text) = ctrl_c() {
            if let Err(err) = tx_d.send(text) {
                panic!("{:?}", err)
            }
        }
    }) {
        panic!("{:?}", err)
    }

    // CTRL+Q launch
    let tx_q = tx.clone();
    if let Err(err) = hk_mng.register(parse_hotkey("CTRL+Q").unwrap(), move || {
        if let Some(text) = ctrl_c() {
            if let Err(err) = tx_q.send(text) {
                panic!("{:?}", err)
            }
        }
    }) {
        panic!("{:?}", err)
    }
}

pub fn ctrl_c() -> Option<String> {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('c'));
    enigo.key_up(Key::Control);
    thread::sleep(Duration::from_millis(50));
    if let Ok(text) = cli_clipboard::get_contents() {
        Some(text)
    } else {
        None
    }
}
