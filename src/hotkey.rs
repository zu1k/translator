use crate::cfg::SETTINGS;
use enigo::{Enigo, Key, KeyboardControllable};
use std::{thread, time::Duration};

#[cfg(target_os = "windows")]
use std::sync::mpsc::SyncSender;
#[cfg(target_os = "windows")]
use tauri_hotkey::{parse_hotkey, HotkeyManager};

pub struct HotkeySetting {
    launch: String,
    quit: String,

    #[cfg(target_os = "windows")]
    hk_mng: HotkeyManager,
}

impl Default for HotkeySetting {
    fn default() -> Self {
        let mut hotkey_settings = Self {
            launch: "ALT+Q".to_string(),
            quit: "CMDORCTRL+SHIFT+D".to_string(),

            #[cfg(target_os = "windows")]
            hk_mng: HotkeyManager::new(),
        };
        let settings = SETTINGS.lock().unwrap();
        if let Ok(launch) = settings.get_string("hotkey.launch") {
            hotkey_settings.set_launch(launch);
        }
        if let Ok(quit) = settings.get_string("hotkey.quit") {
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

    #[cfg(target_os = "windows")]
    pub fn register_hotkey(&mut self, launch_tx: SyncSender<()>) {
        let hk_mng = &mut self.hk_mng;

        // quit
        if let Err(err) = hk_mng.register(parse_hotkey(self.quit.as_str()).unwrap(), move || {
            std::process::exit(0)
        }) {
            panic!("{:?}", err)
        }

        // launch
        if let Err(err) = hk_mng.register(parse_hotkey(self.launch.as_str()).unwrap(), move || {
            launch_tx.send(()).ok();
        }) {
            panic!("{:?}", err)
        }
    }

    #[cfg(target_os = "windows")]
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
