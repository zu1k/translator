mod font;
mod hotkey;
pub mod ui;
pub use hotkey::{ctrl_c, HotkeySetting};
mod toggle_switch;
use config::Config;
use std::sync::RwLock;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}
