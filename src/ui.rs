use crate::{ctrl_c, font, SETTINGS};
use deepl;
use egui::{self, epaint::Color32};
use std::{fmt::Debug, sync::mpsc};

#[cfg(target_os = "windows")]
use crate::HotkeySetting;
#[cfg(target_os = "windows")]
use std::sync::mpsc::Receiver;

#[derive(Debug, Clone, Copy)]
pub struct MouseState {
    last_event: u8,
}

const LINK_COLOR_DOING: Color32 = Color32::GREEN;
const LINK_COLOR_COMMON: Color32 = Color32::GRAY;

impl MouseState {
    fn new() -> Self {
        Self { last_event: 0 }
    }

    fn down(&mut self) {
        self.last_event = 1
    }

    fn moving(&mut self) {
        match self.last_event {
            1 => self.last_event = 2,
            2 => self.last_event = 2,
            _ => self.last_event = 0,
        }
    }

    fn release(&mut self) {
        match self.last_event {
            2 => self.last_event = 3,
            _ => self.last_event = 0,
        }
    }

    fn is_select(&mut self) -> bool {
        if self.last_event == 3 {
            self.last_event = 0;
            true
        } else {
            false
        }
    }
}

pub struct MyApp {
    text: String,
    source_lang: deepl::Lang,
    target_lang: deepl::Lang,

    lang_list_with_auto: Vec<deepl::Lang>,
    lang_list: Vec<deepl::Lang>,
    task_chan: mpsc::SyncSender<(String, deepl::Lang, deepl::Lang)>,
    show_box: bool,
    mouse_state: MouseState,

    event_rx: mpsc::Receiver<Event>,
    clipboard_last: String,
    link_color: Color32,

    #[cfg(target_os = "windows")]
    hk_setting: HotkeySetting,
    #[cfg(target_os = "windows")]
    rx_this: Receiver<String>,
}

pub enum Event {
    TextSet(String),
    MouseEvent(rdev::EventType),
}

impl MyApp {
    pub fn new(
        text: String,
        event_rx: mpsc::Receiver<Event>,
        task_chan: mpsc::SyncSender<(String, deepl::Lang, deepl::Lang)>,
        cc: &eframe::CreationContext<'_>,
    ) -> Self {
        font::install_fonts(&cc.egui_ctx);

        match {
            let settings = SETTINGS.read().unwrap();
            settings
                .get_string("window.theme")
                .unwrap_or("dark".to_string())
                .as_str()
        } {
            "light" => cc.egui_ctx.set_visuals(egui::Visuals::light()),
            _ => cc.egui_ctx.set_visuals(egui::Visuals::dark()),
        }

        #[cfg(target_os = "windows")]
        let (tx, rx) = mpsc::channel();
        #[cfg(target_os = "windows")]
        let mut hk_setting = HotkeySetting::default();
        #[cfg(target_os = "windows")]
        hk_setting.register_hotkey(tx);

        let mut text = text;
        if text.is_empty() {
            text = "请选中需要翻译的文字触发划词翻译".to_string();
        } else {
            let _ = task_chan.send((text.clone(), deepl::Lang::ZH, deepl::Lang::Auto));
        }

        Self {
            text: text.clone(),
            source_lang: deepl::Lang::Auto,
            target_lang: deepl::Lang::ZH,

            lang_list_with_auto: deepl::Lang::lang_list_with_auto(),
            lang_list: deepl::Lang::lang_list(),
            task_chan,
            show_box: false,
            mouse_state: MouseState::new(),
            event_rx,
            clipboard_last: text.clone(),
            link_color: LINK_COLOR_DOING,

            #[cfg(target_os = "windows")]
            hk_setting,
            #[cfg(target_os = "windows")]
            rx_this: rx,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            text,
            source_lang,
            target_lang,
            lang_list_with_auto,
            lang_list,
            task_chan,
            show_box,
            mouse_state,
            event_rx,
            clipboard_last,
            link_color,
            #[cfg(target_os = "windows")]
            hk_setting,
            #[cfg(target_os = "windows")]
            rx_this,
        } = self;
        let old_source_lang = *source_lang;
        let old_target_lang = *target_lang;

        if ctx.input().key_pressed(egui::Key::Escape) {
            #[cfg(target_os = "windows")]
            hk_setting.unregister_all();
            frame.quit()
        }

        while let Ok(event) = event_rx.try_recv() {
            match event {
                Event::TextSet(text_new) => {
                    *link_color = LINK_COLOR_COMMON;
                    *text = text_new;
                }
                Event::MouseEvent(mouse_event) => match mouse_event {
                    rdev::EventType::ButtonPress(button) => {
                        if button == rdev::Button::Left {
                            mouse_state.down()
                        }
                    }
                    rdev::EventType::ButtonRelease(button) => {
                        if button == rdev::Button::Left {
                            mouse_state.release()
                        }
                    }
                    rdev::EventType::MouseMove { x: _, y: _ } => mouse_state.moving(),
                    _ => {}
                },
            }
        }

        if mouse_state.is_select() && !ctx.input().pointer.has_pointer() {
            if let Some(text_new) = ctrl_c() {
                if text_new.ne(clipboard_last) {
                    *clipboard_last = text_new.clone();
                    *text = text_new.clone();
                    *link_color = LINK_COLOR_DOING;
                    let _ = task_chan.send((text_new, *target_lang, *source_lang));
                }
            }
        }

        #[cfg(target_os = "windows")]
        if let Ok(text_new) = rx_this.try_recv() {
            *text = text_new.clone();
            *link_color = LINK_COLOR_DOING;
            let _ = task_chan.send((text_new, *target_lang, *source_lang));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.horizontal_top(|ui| {
                    let combobox_width = 145.0;
                    egui::ComboBox::from_id_source(egui::Id::new("source_lang_ComboBox"))
                        .selected_text(source_lang.description())
                        .width(combobox_width)
                        .show_ui(ui, |ui| {
                            for i in lang_list_with_auto {
                                let i = i.to_owned();
                                ui.selectable_value(source_lang, i, i.description());
                            }
                        });

                    if ui.add(egui::Button::new(" ⇌ ").frame(false)).clicked() {
                        let tmp_target_lang = *target_lang;
                        *target_lang = if *source_lang == deepl::Lang::Auto {
                            deepl::Lang::EN
                        } else {
                            *source_lang
                        };
                        *source_lang = tmp_target_lang;
                    };

                    egui::ComboBox::from_id_source(egui::Id::new("target_lang_ComboBox"))
                        .selected_text(target_lang.description())
                        .width(combobox_width)
                        .show_ui(ui, |ui| {
                            for i in lang_list {
                                let i = i.to_owned();
                                ui.selectable_value(target_lang, i, i.description());
                            }
                        });
                    if ui.add(egui::Button::new("翻译")).clicked() {
                        let _ = task_chan.send((text.clone(), *target_lang, *source_lang));
                        *link_color = LINK_COLOR_DOING;
                    };

                    ui.horizontal_wrapped(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                            ui.visuals_mut().hyperlink_color = *link_color;
                            ui.hyperlink_to(
                                egui::special_emojis::GITHUB.to_string(),
                                "https://github.com/zu1k/copy-translator",
                            );

                            if ui.add(egui::Button::new("□").frame(false)).clicked() {
                                *show_box = !*show_box;
                                frame.set_decorations(*show_box);
                            };
                            if ui
                                .add(egui::Button::new("○").frame(false))
                                .is_pointer_button_down_on()
                            {
                                frame.drag_window();
                            };
                        });
                    });

                    if *source_lang != old_source_lang || *target_lang != old_target_lang {
                        *link_color = LINK_COLOR_DOING;
                        let _ = task_chan.send((text.clone(), *target_lang, *source_lang));
                    };
                });

                ui.separator();

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(text)
                                .desired_width(2000.0)
                                .desired_rows(7)
                                .frame(false)
                                .lock_focus(true),
                        )
                    });
            });
        });
        ctx.request_repaint();

        #[cfg(windows)]
        frame.set_window_size(ctx.used_size());
    }
}
