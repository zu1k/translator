use std::sync::Mutex;

use config::Config;
use log::warn;

lazy_static! {
    pub static ref SETTINGS: Mutex<Config> = Mutex::new(Config::default());
}

pub fn init_config() {
    #[cfg(not(target_os = "windows"))]
    let settings_path = std::path::PathBuf::from("/etc/translator/settings");

    #[cfg(target_os = "windows")]
    let settings_path = {
        std::env::current_exe()
            .map(|path| match path.parent() {
                Some(parent) => parent.join("settings"),
                None => std::path::PathBuf::from("settings"),
            })
            .unwrap_or(std::path::PathBuf::from("settings"))
    };

    let builder =
        Config::builder().add_source(config::File::with_name(settings_path.to_str().unwrap()));
    match builder.build() {
        Ok(config) => *SETTINGS.lock().unwrap() = config,
        Err(err) => warn!("settings merge failed, use default settings, err: {}", err),
    }
}

pub fn get_api() -> String {
    let settings = SETTINGS.lock().unwrap();
    settings
        .get_string("api")
        .unwrap_or("https://deepl.zu1k.com/translate".to_string())
}

pub fn get_window_size() -> (f32, f32) {
    let settings = SETTINGS.lock().unwrap();
    (
        settings.get_float("window.size.width").unwrap_or(500.0) as f32,
        settings.get_float("window.size.height").unwrap_or(200.0) as f32,
    )
}

pub fn get_theme() -> String {
    let settings = SETTINGS.lock().unwrap();
    settings
        .get_string("window.theme")
        .unwrap_or("dark".to_string())
}
