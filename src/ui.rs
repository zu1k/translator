use crate::{ctrl_c, font};
use deepl;
use eframe::{egui, epi};
use std::{fmt::Debug, sync::mpsc};

#[cfg(target_os = "windows")]
use crate::HotkeySetting;
#[cfg(target_os = "windows")]
use std::sync::mpsc::Receiver;

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
    task_chan: mpsc::SyncSender<(String, deepl::Lang, Option<deepl::Lang>)>,
    show_box: bool,
    mouse_state: MouseState,

    event_rx: mpsc::Receiver<Event>,
    clipboard_last: String,

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
        task_chan: mpsc::SyncSender<(String, deepl::Lang, Option<deepl::Lang>)>,
    ) -> Self {
        #[cfg(target_os = "windows")]
        {
            let (tx, rx) = mpsc::channel();
            let mut hk_setting = HotkeySetting::default();
            hk_setting.register_hotkey(tx);
        }
        Self {
            text,
            source_lang: deepl::Lang::Auto,
            target_lang: deepl::Lang::ZH,

            lang_list_with_auto: deepl::Lang::lang_list_with_auto(),
            lang_list: deepl::Lang::lang_list(),
            task_chan,
            show_box: false,
            mouse_state: MouseState::new(),
            event_rx,
            clipboard_last: String::new(),

            #[cfg(target_os = "windows")]
            hk_setting,
            #[cfg(target_os = "windows")]
            rx_this: rx,
        }
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

        if self.text.is_empty() {
            self.text = "请选中需要翻译的文字触发划词翻译".to_string();
        } else {
            let _ =
                self.task_chan
                    .send((self.text.clone(), self.target_lang, Some(self.source_lang)));
            self.clipboard_last = self.text.clone();
            self.text = "正在翻译中...\r\n\r\n".to_string() + &self.text;
        }
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
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
                    let _ = task_chan.send((text_new, *target_lang, Some(*source_lang)));
                }
            }
        }

        #[cfg(target_os = "windows")]
        if let Ok(text_new) = rx_this.try_recv() {
            *text = "正在翻译中...\r\n\r\n".to_string() + &text_new;
            let _ = task_chan.send((text_new, *target_lang, *source_lang));
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

                    ui.vertical_centered_justified(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                            ui.hyperlink_to(
                                format!("{} GitHub", egui::special_emojis::GITHUB),
                                "https://github.com/zu1k/copy-translator",
                            );

                            let drag_button = ui.add(egui::Button::new("○").frame(false));
                            if drag_button.clicked() {
                                *show_box = !*show_box;
                            } else if ctx.input().key_pressed(egui::Key::D)
                                && drag_button.is_pointer_button_down_on()
                            {
                                frame.drag_window();
                            }
                        });
                    });

                    if *source_lang != old_source_lang || *target_lang != old_target_lang {
                        let _ = task_chan.send((text.clone(), *target_lang, Some(*source_lang)));
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

        frame.set_decorations(*show_box);
        ctx.request_repaint();
    }
}
