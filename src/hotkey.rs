use crate::SETTINGS;
use enigo::{Enigo, Key, KeyboardControllable};
use std::{sync::mpsc::Sender, thread, time::Duration};
use tauri_hotkey::{parse_hotkey, HotkeyManager};

pub struct HotkeySetting {
    launch: String,
    quit: String,
    hk_mng: HotkeyManager,
}

impl Default for HotkeySetting {
    fn default() -> Self {
        let mut hotkey_settings = Self {
            launch: "CMDORCTRL+D".to_string(),
            quit: "CMDORCTRL+SHIFT+D".to_string(),
            hk_mng: HotkeyManager::new(),
        };
        let settings = SETTINGS.read().unwrap();
        if let Ok(launch) = settings.get_str("hotkey.launch") {
            hotkey_settings.set_launch(launch);
        }
        if let Ok(quit) = settings.get_str("hotkey.quit") {
            hotkey_settings.set_quit(quit);
        }
        hotkey_settings
    }
}

impl HotkeySetting {
    pub fn set_launch(&mut self, s: String) {
        self.launch = s;
    }

    pub fn set_quit(&mut self, s: String) {
        self.quit = s;
    }

    pub fn register_hotkey(&mut self, tx: Sender<String>) {
        let hk_mng = &mut self.hk_mng;

        // quit
        if let Err(err) = hk_mng.register(parse_hotkey(self.quit.as_str()).unwrap(), move || {
            std::process::exit(0)
        }) {
            panic!("{:?}", err)
        }

        // launch
        let tx_d = tx;
        if let Err(err) = hk_mng.register(parse_hotkey(self.launch.as_str()).unwrap(), move || {
            if let Some(text) = ctrl_c() {
                if let Err(err) = tx_d.send(text) {
                    panic!("{:?}", err)
                }
            }
        }) {
            panic!("{:?}", err)
        }
    }

    pub fn unregister_all(&mut self) {
        let _ = self.hk_mng.unregister_all();
    }
}

pub fn ctrl_c() -> Option<String> {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('c'));
    enigo.key_up(Key::Control);
    thread::sleep(Duration::from_millis(200));
    if let Ok(text) = cli_clipboard::get_contents() {
        Some(text)
    } else {
        None
    }
}
