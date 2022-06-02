use eframe::{self, App, CreationContext};
use egui::Color32;
use log::*;
use std::{
    io::Cursor,
    sync::{mpsc, Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use crate::{
    cfg::{get_api, get_window_size, init_config},
    hotkey::ctrl_c,
    mouse::MouseState,
    ui,
};

const LINK_COLOR_DOING: Color32 = Color32::GREEN;
const LINK_COLOR_COMMON: Color32 = Color32::GRAY;

fn init_ui(cc: &CreationContext) -> Box<dyn App> {
    let (task_tx, task_rx) = mpsc::sync_channel(1);

    let ctx = cc.egui_ctx.clone();

    let text = Arc::new(Mutex::new("请选中需要翻译的文字触发划词翻译".to_string()));
    let source_lang = Arc::new(Mutex::new(deepl::Lang::Auto));
    let target_lang = Arc::new(Mutex::new(deepl::Lang::ZH));
    let link_color = Arc::new(Mutex::new(LINK_COLOR_COMMON));

    // first
    {
        let text = text.clone();
        let source_lang = source_lang.clone();
        let target_lang = target_lang.clone();
        let link_color = link_color.clone();
        thread::spawn(move || {
            if let Some(text_first) = ctrl_c() {
                let text_first = text_first.trim();
                if text_first.len() > 0 {
                    // 新翻译任务 UI
                    {
                        *text.lock().unwrap() = text_first.to_string();
                        *link_color.lock().unwrap() = LINK_COLOR_DOING;
                    }

                    // 开始翻译
                    let result = {
                        let target_lang = target_lang.lock().unwrap().to_owned();
                        let source_lang = source_lang.lock().unwrap().to_owned();
                        deepl::translate(
                            &get_api(),
                            text_first.to_string(),
                            target_lang,
                            source_lang,
                        )
                        .unwrap_or("翻译接口失效，请更换".to_string())
                    };

                    // 翻译结束 UI
                    {
                        *text.lock().unwrap() = result;
                        *link_color.lock().unwrap() = LINK_COLOR_COMMON;
                    }
                }
            }
        });
    }

    // 监听鼠标动作
    {
        let text = text.clone();
        let source_lang = source_lang.clone();
        let target_lang = target_lang.clone();
        let link_color = link_color.clone();
        let mouse_state = Arc::new(Mutex::new(MouseState::new()));

        {
            let mouse_state = mouse_state.clone();
            thread::spawn(move || {
                if let Err(err) = rdev::listen(move |event| {
                    match event.event_type {
                        rdev::EventType::ButtonPress(button) => {
                            if button == rdev::Button::Left {
                                mouse_state.lock().unwrap().down();
                            }
                        }
                        rdev::EventType::ButtonRelease(button) => {
                            if button == rdev::Button::Left {
                                mouse_state.lock().unwrap().release()
                            }
                        }
                        rdev::EventType::MouseMove { x: _, y: _ } => {
                            mouse_state.lock().unwrap().moving()
                        }
                        _ => {}
                    };
                }) {
                    warn!("rdev listen error: {:?}", err)
                }
            });
        }

        {
            let mouse_state = mouse_state.clone();
            thread::spawn(move || {
                let mut clipboard_last = "".to_string();
                loop {
                    if mouse_state.lock().unwrap().is_select() && !ctx.input().pointer.has_pointer()
                    {
                        if let Some(text_new) = ctrl_c() {
                            if text_new != clipboard_last {
                                clipboard_last = text_new.clone();
                                // 新翻译任务 UI
                                {
                                    *text.lock().unwrap() = text_new.clone();
                                    *link_color.lock().unwrap() = LINK_COLOR_DOING;
                                }

                                // 开始翻译
                                let result = {
                                    let target_lang = target_lang.lock().unwrap().to_owned();
                                    let source_lang = source_lang.lock().unwrap().to_owned();
                                    deepl::translate(&get_api(), text_new, target_lang, source_lang)
                                        .unwrap_or("翻译接口失效，请更换".to_string())
                                };

                                // 翻译结束 UI
                                {
                                    *text.lock().unwrap() = result;
                                    *link_color.lock().unwrap() = LINK_COLOR_COMMON;
                                }
                            }
                        }
                    }
                    sleep(Duration::from_millis(100));
                }
            });
        }
    }

    // 监听翻译按钮触发
    {
        let text = text.clone();
        let source_lang = source_lang.clone();
        let target_lang = target_lang.clone();
        let link_color = link_color.clone();
        thread::spawn(move || {
            loop {
                task_rx.recv().ok();
                {
                    // 新翻译任务 UI
                    *link_color.lock().unwrap() = LINK_COLOR_DOING;
                    let text_to_trans = text.lock().unwrap().to_owned();

                    // 开始翻译
                    let result = {
                        let target_lang = target_lang.clone().lock().unwrap().to_owned();
                        let source_lang = source_lang.lock().unwrap().to_owned();
                        deepl::translate(&get_api(), text_to_trans, target_lang, source_lang)
                            .unwrap_or("翻译接口失效，请更换".to_string())
                    };

                    // 翻译结束 UI
                    {
                        *text.lock().unwrap() = result;
                        *link_color.lock().unwrap() = LINK_COLOR_COMMON;
                    }
                }
            }
        });
    }

    Box::new(ui::MyApp::new(
        text,
        source_lang,
        target_lang,
        link_color,
        task_tx,
        cc,
    ))
}

pub fn run() {
    init_config();
    let (width, height) = get_window_size();

    // embed ico file
    let ioc_buf = Cursor::new(include_bytes!("../res/copy-translator.ico"));
    let icon_dir = ico::IconDir::read(ioc_buf).unwrap();
    let image = icon_dir.entries()[5].decode().unwrap();
    let ico_data = eframe::IconData {
        rgba: std::vec::Vec::from(image.rgba_data()),
        width: image.width(),
        height: image.height(),
    };

    let native_options = eframe::NativeOptions {
        always_on_top: true,
        decorated: false,
        initial_window_size: Some(egui::vec2(width, height)),
        icon_data: Some(ico_data),
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native("Copy Translator", native_options, Box::new(init_ui));
}
