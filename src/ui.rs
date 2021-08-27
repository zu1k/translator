use crate::font;

use crate::{ctrl_c, register_hotkey};
use deepl;
use eframe::{egui, epi};
use std::fmt::Debug;
use std::sync::mpsc::{self, Receiver, Sender};
use tauri_hotkey::HotkeyManager;

#[derive(Debug, Clone, Copy)]
pub struct MouseState {
    last_event: u8,
}

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
    hk_mng: HotkeyManager,
    tx_this: Sender<String>,
    rx_this: Receiver<String>,
    show_box: bool,
    mouse_state: MouseState,

    event_rx: mpsc::Receiver<Event>,
    clipboard_last: String,
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
    ) -> Self {
        let (tx, rx) = mpsc::channel();
        let mut s = Self {
            text,
            source_lang: deepl::Lang::Auto,
            target_lang: deepl::Lang::ZH,

            lang_list_with_auto: deepl::Lang::lang_list_with_auto(),
            lang_list: deepl::Lang::lang_list(),
            task_chan,
            hk_mng: HotkeyManager::new(),
            tx_this: tx,
            rx_this: rx,
            show_box: false,
            mouse_state: MouseState::new(),
            event_rx,
            clipboard_last: String::new(),
        };
        register_hotkey(&mut s.hk_mng, s.tx_this.clone());
        s
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Copy Translator"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        font::install_fonts(_ctx);
        let _ = self.task_chan.send((
            self.text.clone(),
            self.target_lang.clone(),
            self.source_lang.clone(),
        ));
        self.clipboard_last = self.text.clone();
        self.text = "正在翻译中...\r\n\r\n".to_string() + &self.text;
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            text,
            source_lang,
            target_lang,
            lang_list_with_auto,
            lang_list,
            task_chan,
            hk_mng: _,
            tx_this: _,
            rx_this,
            show_box,
            mouse_state,
            event_rx,
            clipboard_last,
        } = self;
        let old_source_lang = source_lang.clone();
        let old_target_lang = target_lang.clone();

        if ctx.input().key_pressed(egui::Key::Escape) {
            frame.quit()
        }

        while let Ok(event) = event_rx.try_recv() {
            match event {
                Event::TextSet(text_new) => {
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
                    *text = "正在翻译中...\r\n\r\n".to_string() + &text_new;
                    let _ = task_chan.send((
                        text_new.clone(),
                        target_lang.clone(),
                        source_lang.clone(),
                    ));
                }
            }
        }

        if let Ok(text_new) = rx_this.try_recv() {
            *text = "正在翻译中...\r\n\r\n".to_string() + &text_new;
            let _ = task_chan.send((text_new.clone(), target_lang.clone(), source_lang.clone()));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.horizontal_wrapped(|ui| {
                    let combobox_width = 120.0;
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
                        let tmp_target_lang = target_lang.clone();
                        *target_lang = if *source_lang == deepl::Lang::Auto {
                            deepl::Lang::EN
                        } else {
                            source_lang.clone()
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
                    ui.scope(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                            ui.hyperlink_to(
                                format!("{} GitHub", egui::special_emojis::GITHUB),
                                "https://github.com/zu1k/copy-translator",
                            );

                            ui.add(super::toggle_switch::toggle(show_box))
                                .on_hover_text("显示窗口框，用来修改窗口大小和移动位置");
                        });
                    });

                    if source_lang.clone() != old_source_lang
                        || target_lang.clone() != old_target_lang
                    {
                        let _ = task_chan.send((
                            text.clone(),
                            target_lang.clone(),
                            source_lang.clone(),
                        ));
                    };
                });

                ui.separator();

                egui::ScrollArea::auto_sized().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(text)
                            .desired_width(2000.0)
                            .desired_rows(7)
                            .lock_focus(true),
                    );
                });
            });
        });
        frame.set_window_size(ctx.used_size());
        frame.set_decorations(*show_box);

        // repaint everytime otherwise other events are needed to trigger
        ctx.request_repaint();
    }
}
