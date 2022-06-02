use eframe::{self, App, CreationContext};
use log::*;
use std::{
    sync::{mpsc, Arc, Mutex},
    thread::{self, sleep},
    time::Duration,
};

use crate::{
    cfg::{get_api, get_window_size, init_config},
    hotkey::ctrl_c,
    mouse::MouseState,
    ui::{self, get_icon_data, State, LINK_COLOR_COMMON, LINK_COLOR_DOING},
};

fn setup_ui_task(cc: &CreationContext) -> Box<dyn App> {
    let (task_tx, task_rx) = mpsc::sync_channel(1);

    let ctx = cc.egui_ctx.clone();

    let state = Arc::new(Mutex::new(State {
        text: "请选中需要翻译的文字触发划词翻译".to_string(),
        source_lang: deepl::Lang::Auto,
        target_lang: deepl::Lang::ZH,
        link_color: LINK_COLOR_COMMON,
    }));

    // first
    {
        let state = state.clone();
        thread::spawn(move || {
            if let Some(text_first) = ctrl_c() {
                let text_first = text_first.trim();
                if text_first.len() > 0 {
                    // 新翻译任务 UI
                    {
                        let mut state = state.lock().unwrap();
                        state.text = text_first.to_string();
                        state.link_color = LINK_COLOR_DOING;
                    }

                    // 开始翻译
                    let result = {
                        let (source_lang, target_lang) = {
                            let state = state.lock().unwrap();
                            (state.source_lang, state.target_lang)
                        };
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
                        let mut state = state.lock().unwrap();
                        state.text = result;
                        state.link_color = LINK_COLOR_COMMON;
                    }
                }
            }
        });
    }

    // 监听鼠标动作
    {
        let state = state.clone();
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
                                    let mut state = state.lock().unwrap();
                                    state.text = text_new.clone();
                                    state.link_color = LINK_COLOR_DOING;
                                }

                                // 开始翻译
                                let result = {
                                    let (source_lang, target_lang) = {
                                        let state = state.lock().unwrap();
                                        (state.source_lang, state.target_lang)
                                    };
                                    deepl::translate(&get_api(), text_new, target_lang, source_lang)
                                        .unwrap_or("翻译接口失效，请更换".to_string())
                                };

                                // 翻译结束 UI
                                {
                                    let mut state = state.lock().unwrap();
                                    state.text = result;
                                    state.link_color = LINK_COLOR_COMMON;
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
        let state = state.clone();
        thread::spawn(move || {
            loop {
                task_rx.recv().ok();
                {
                    // 新翻译任务 UI
                    {
                        let mut state = state.lock().unwrap();
                        state.link_color = LINK_COLOR_DOING;
                    }

                    // 开始翻译
                    let result = {
                        let (text, source_lang, target_lang) = {
                            let state = state.lock().unwrap();
                            (state.text.clone(), state.source_lang, state.target_lang)
                        };
                        deepl::translate(&get_api(), text, target_lang, source_lang)
                            .unwrap_or("翻译接口失效，请更换".to_string())
                    };

                    // 翻译结束 UI
                    {
                        let mut state = state.lock().unwrap();
                        state.text = result;
                        state.link_color = LINK_COLOR_COMMON;
                    }
                }
            }
        });
    }

    Box::new(ui::MyApp::new(state, task_tx, cc))
}

pub fn run() {
    init_config();
    let (width, height) = get_window_size();

    let native_options = eframe::NativeOptions {
        always_on_top: true,
        decorated: false,
        initial_window_size: Some(egui::vec2(width, height)),
        icon_data: Some(get_icon_data()),
        drag_and_drop_support: true,
        ..Default::default()
    };
    eframe::run_native("Copy Translator", native_options, Box::new(setup_ui_task));
}
