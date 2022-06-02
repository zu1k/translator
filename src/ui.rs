use crate::{cfg::get_theme, font};
use deepl;
use egui::{self, epaint::Color32};
use std::sync::{mpsc, Arc, Mutex};

#[cfg(target_os = "windows")]
use crate::HotkeySetting;
#[cfg(target_os = "windows")]
use std::sync::mpsc::Receiver;

pub struct MyApp {
    text: Arc<Mutex<String>>,
    source_lang: Arc<Mutex<deepl::Lang>>,
    target_lang: Arc<Mutex<deepl::Lang>>,
    link_color: Arc<Mutex<Color32>>,

    lang_list_with_auto: Vec<deepl::Lang>,
    lang_list: Vec<deepl::Lang>,
    task_chan: mpsc::SyncSender<()>,
    show_box: bool,

    #[cfg(target_os = "windows")]
    hk_setting: HotkeySetting,
    #[cfg(target_os = "windows")]
    rx_this: Receiver<String>,
}

impl MyApp {
    pub fn new(
        text: Arc<Mutex<String>>,
        source_lang: Arc<Mutex<deepl::Lang>>,
        target_lang: Arc<Mutex<deepl::Lang>>,
        link_color: Arc<Mutex<Color32>>,
        task_chan: mpsc::SyncSender<()>,
        cc: &eframe::CreationContext<'_>,
    ) -> Self {
        font::install_fonts(&cc.egui_ctx);

        match get_theme().as_str() {
            "light" => cc.egui_ctx.set_visuals(egui::Visuals::light()),
            _ => cc.egui_ctx.set_visuals(egui::Visuals::dark()),
        }

        #[cfg(target_os = "windows")]
        let (tx, rx) = mpsc::channel();
        #[cfg(target_os = "windows")]
        let mut hk_setting = HotkeySetting::default();
        #[cfg(target_os = "windows")]
        hk_setting.register_hotkey(tx);

        Self {
            text,
            source_lang,
            target_lang,
            link_color,

            lang_list_with_auto: deepl::Lang::lang_list_with_auto(),
            lang_list: deepl::Lang::lang_list(),
            task_chan,
            show_box: false,

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
            link_color,

            lang_list_with_auto,
            lang_list,
            task_chan,
            show_box,
            #[cfg(target_os = "windows")]
            hk_setting,
            #[cfg(target_os = "windows")]
            rx_this,
        } = self;
        let mut old_source_lang = *source_lang.lock().unwrap();
        let mut old_target_lang = *target_lang.lock().unwrap();

        if ctx.input().key_pressed(egui::Key::Escape) {
            #[cfg(target_os = "windows")]
            hk_setting.unregister_all();
            frame.quit()
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
                        .selected_text(old_source_lang.description())
                        .width(combobox_width)
                        .show_ui(ui, |ui| {
                            for i in lang_list_with_auto {
                                let i = i.to_owned();
                                ui.selectable_value(&mut old_source_lang, i, i.description());
                            }
                        });

                    if ui.add(egui::Button::new(" ⇌ ").frame(false)).clicked() {
                        let tmp_target_lang = old_target_lang.clone();
                        let tmp_source_lang = old_source_lang.clone();
                        old_target_lang = if tmp_source_lang == deepl::Lang::Auto {
                            deepl::Lang::EN
                        } else {
                            tmp_source_lang
                        };
                        old_source_lang = tmp_target_lang;
                    };

                    egui::ComboBox::from_id_source(egui::Id::new("target_lang_ComboBox"))
                        .selected_text(old_target_lang.description())
                        .width(combobox_width)
                        .show_ui(ui, |ui| {
                            for i in lang_list {
                                let i = i.to_owned();
                                ui.selectable_value(&mut old_target_lang, i, i.description());
                            }
                        });
                    if ui.add(egui::Button::new("翻译")).clicked() {
                        let _ = task_chan.send(());
                    };

                    ui.horizontal_wrapped(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(), |ui| {
                            ui.visuals_mut().hyperlink_color = *link_color.lock().unwrap();
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
                });

                ui.separator();

                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut *text.lock().unwrap())
                                .desired_width(2000.0)
                                .desired_rows(7)
                                .frame(false)
                                .lock_focus(true),
                        )
                    });
            });
        });

        {
            let mut source_lang = source_lang.lock().unwrap();
            let mut target_lang = target_lang.lock().unwrap();

            if *source_lang != old_source_lang || *target_lang != old_target_lang {
                *source_lang = old_source_lang;
                *target_lang = old_target_lang;
                let _ = task_chan.send(());
            };
        }

        ctx.request_repaint();

        #[cfg(windows)]
        frame.set_window_size(ctx.used_size());
    }
}
