mod font;
mod hotkey;
pub mod ui;
use config::Config;
pub use hotkey::{ctrl_c, HotkeySetting};
use std::sync::RwLock;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref SETTINGS: RwLock<Config> = RwLock::new(Config::default());
}
