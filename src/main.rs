#![windows_subsystem = "windows"]
#![cfg_attr(not(debug_assertions), deny(warnings))]

use copy_translator::{ui, SETTINGS};
use log::*;
use std::{io::Cursor, sync::mpsc, thread};

cfg_if::cfg_if! {
    if #[cfg(target_os="windows")] {
        use copy_translator::HotkeySetting;
        fn run() {
            if let Ok(path) = std::env::current_exe() {
                let settings_path = match path.parent() {
                    Some(parent) => parent.join("settings"),
                    None => std::path::PathBuf::from("settings"),
                };
                let mut settings = SETTINGS.write().unwrap();
                if let Err(err) = settings.merge(config::File::from(settings_path)) {
                    warn!("settings merge failed, use default settings, err: {}", err);
                }
            }

            let (width, height) = {
                let settings = SETTINGS.read().unwrap();
                (settings.get_float("window.size.width").unwrap_or(500.0) as f32,
                settings.get_float("window.size.height").unwrap_or(200.0) as f32)
            };

            let (tx_hk, rx) = mpsc::channel();

            let mut hotkey_settings = HotkeySetting::default();
            hotkey_settings.register_hotkey(tx_hk.clone());

            // embed ico file
            let ioc_buf = Cursor::new(include_bytes!("../res/copy-translator.ico"));
            let icon_dir = ico::IconDir::read(ioc_buf).unwrap();
            let image = icon_dir.entries()[5].decode().unwrap();
            let ico_data = epi::IconData {
                rgba: std::vec::Vec::from(image.rgba_data()),
                width: image.width(),
                height: image.height(),
            };

            // listen for global mouse event
            let (rdev_tx, rdev_rx) = mpsc::sync_channel(1);
            let mouse_event_rx_wrap = std::sync::Arc::new(std::sync::Mutex::new(rdev_rx));
            thread::spawn(move || {
                // let last_move =
                if let Err(error) = rdev::listen(move |event| {
                    match event.event_type {
                        rdev::EventType::ButtonPress(button) => {
                            if button == rdev::Button::Left {
                                let _ = rdev_tx.try_send(ui::Event::MouseEvent(event.event_type));
                            }
                        }
                        rdev::EventType::ButtonRelease(button) => {
                            if button == rdev::Button::Left {
                                let _ = rdev_tx.try_send(ui::Event::MouseEvent(event.event_type));
                            }
                        }
                        rdev::EventType::MouseMove { x: _, y: _ } => {
                            let _ = rdev_tx.try_send(ui::Event::MouseEvent(event.event_type));
                        }
                        _ => {}
                    };
                }) {
                    warn!("rdev listen error: {:?}", error)
                }
            });

            loop {
                match rx.recv() {
                    Ok(text) => {
                        let (event_tx, event_rx) = mpsc::sync_channel(1);
                        let (task_tx, task_rx) = mpsc::sync_channel(1);

                        let event_tx_trasnlate = event_tx.clone();
                        thread::spawn(move || {
                            while let Ok((text, target_lang, source_lang)) = task_rx.recv() {
                                let _ = match deepl::translate(text, target_lang, source_lang) {
                                    Ok(text) => event_tx_trasnlate.send(ui::Event::TextSet(text)),
                                    Err(_err) => event_tx_trasnlate
                                        .send(ui::Event::TextSet("翻译接口失效，请更新最新版".into())),
                                };
                            }
                        });

                        let mouse_event_rx = mouse_event_rx_wrap.clone();
                        let event_tx_mouse = event_tx.clone();
                        thread::spawn(move || {
                            loop {
                                let rx = mouse_event_rx.lock().unwrap();
                                match rx.recv() {
                                    Ok(event) => {
                                        if event_tx_mouse.send(event).is_err() {
                                            break;
                                        }
                                    }
                                    Err(_) => break,
                                }
                            }
                        });
                        hotkey_settings.unregister_all();
                        let app = ui::MyApp::new(text, event_rx, task_tx);
                        let app = Box::new(app);
                        let native_options = eframe::NativeOptions {
                            always_on_top: true,
                            decorated: false,
                            initial_window_size: Some(egui::vec2(width, height)),
                            icon_data: Some(ico_data.clone()),
                            drag_and_drop_support: true,
                            ..Default::default()
                        };
                        eframe::run_native(app, native_options);
                        hotkey_settings.register_hotkey(tx_hk.clone());
                    }
                    Err(err) => {
                        panic!("{}", err)
                    }
                }
            }
        }
    } else {
        use copy_translator::ctrl_c;
        fn run() {
            {
                let mut settings = SETTINGS.write().unwrap();
                if let Err(err) = settings.merge(config::File::with_name("/etc/copy-translator/settings")) {
                    warn!("settings merge failed, use default settings, err: {}", err);
                }
            }

            let (width, height) = {
                let settings = SETTINGS.read().unwrap();
                (settings.get_float("window.size.width").unwrap_or(500.0) as f32,
                settings.get_float("window.size.height").unwrap_or(200.0) as f32)
            };

            let (event_tx, event_rx) = mpsc::sync_channel(1);
            let (task_tx, task_rx) = mpsc::sync_channel(1);

            // 翻译线程
            let event_tx_trasnlate = event_tx.clone();
            thread::spawn(move || {
                while let Ok((text, target_lang, source_lang)) = task_rx.recv() {
                    let _ = match deepl::translate(text, target_lang, source_lang) {
                        Ok(text) => event_tx_trasnlate.try_send(ui::Event::TextSet(text)),
                        Err(_err) => {
                            event_tx_trasnlate.try_send(ui::Event::TextSet("翻译接口失效，请更新最新版".into()))
                        }
                    };
                }
            });

            // listen for global mouse event
            thread::spawn(move || {
                // let last_move =
                if let Err(error) = rdev::listen(move |event| {
                    match event.event_type {
                        rdev::EventType::ButtonPress(button) => {
                            if button == rdev::Button::Left {
                                let _ = event_tx.try_send(ui::Event::MouseEvent(event.event_type));
                            }
                        }
                        rdev::EventType::ButtonRelease(button) => {
                            if button == rdev::Button::Left {
                                let _ = event_tx.try_send(ui::Event::MouseEvent(event.event_type));
                            }
                        }
                        rdev::EventType::MouseMove { x: _, y: _ } => {
                            let _ = event_tx.try_send(ui::Event::MouseEvent(event.event_type));
                        }
                        _ => {}
                    };
                }) {
                    warn!("rdev listen error: {:?}", error)
                }
            });

            let text = match ctrl_c() {
                Some(text) => text,
                None => "".into()
            };

            // embed ico file
            let ioc_buf = Cursor::new(include_bytes!("../res/copy-translator.ico"));
            let icon_dir = ico::IconDir::read(ioc_buf).unwrap();
            let image = icon_dir.entries()[5].decode().unwrap();
            let ico_data = epi::IconData {
                rgba: std::vec::Vec::from(image.rgba_data()),
                width: image.width(),
                height: image.height(),
            };

            let app = ui::MyApp::new(text, event_rx, task_tx);
            let app = Box::new(app);
            let native_options = eframe::NativeOptions {
                always_on_top: true,
                decorated: false,
                initial_window_size: Some(egui::vec2(width, height)),
                icon_data: Some(ico_data),
                drag_and_drop_support: true,
                ..Default::default()
            };
            eframe::run_native(app, native_options);
        }
    }
}

fn main() {
    run()
}
