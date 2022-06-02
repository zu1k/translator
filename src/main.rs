#![cfg_attr(not(debug_assertions), deny(warnings), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate cfg_if;

mod cfg;
mod font;
mod hotkey;
mod mouse;
mod ui;

cfg_if! {
    if #[cfg(target_os="windows")] {
        mod windows;
        use windows::*;
    } else {
        mod unix;
        use unix::*;
    }
}

fn main() {
    run()
}
