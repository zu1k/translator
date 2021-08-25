use crate::font;

use eframe::{egui, epi};
use online_api as deepl;

pub struct MyApp {
    text: String,
    source_lang: deepl::Lang,
    target_lang: deepl::Lang,

    lang_list_with_auto: Vec<deepl::Lang>,
    lang_list: Vec<deepl::Lang>,
    new_text: Box<Option<String>>
}

impl MyApp {
    pub fn new(text: String) -> Self {
        Self {
            text,
            source_lang: deepl::Lang::Auto,
            target_lang: deepl::Lang::ZH,

            lang_list_with_auto: deepl::Lang::lang_list_with_auto(),
            lang_list: deepl::Lang::lang_list(),
            new_text: Box::new(None)
        }
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Copy Translator"
    }

    fn setup(&mut self, _ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>, _storage: Option<&dyn epi::Storage>) {
        font::install_fonts(_ctx);        
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { text, source_lang, target_lang, lang_list_with_auto, lang_list, new_text } = self;
        let old_source_lang = source_lang.clone();
        let old_target_lang = target_lang.clone();

        if ctx.input().key_pressed(egui::Key::Escape) {
            frame.quit()
        }

        let len = text.len();
        println!("{}", len);
        // let width = if len<400 {
        //     800.0
        // } else if len<1000 {
        //     1000.0
        // }  else if len<2000 {
        //     1600.0
        // } else {
        //     2000.0
        // };
        let width = 600.0;

        println!("{}", width);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_source(egui::Id::new("source_lang_ComboBox"))
                    .selected_text(source_lang.description())
                    .show_ui(ui, |ui| {
                        for i in lang_list_with_auto {
                            let i = i.to_owned();
                            ui.selectable_value(source_lang, i, i.description());
                        }
                    });
                if ui.add(egui::Button::new("交换")).clicked() {
                    let tmp_target_lang = target_lang.clone();
                    *target_lang = source_lang.clone();
                    *source_lang = tmp_target_lang;
                };
                egui::ComboBox::from_id_source(egui::Id::new("target_lang_ComboBox"))
                .selected_text(target_lang.description())
                .show_ui(ui, |ui| {
                    for i in lang_list {
                        let i = i.to_owned();
                        ui.selectable_value(target_lang, i, i.description());
                    }
                });

                if source_lang.clone()!=old_source_lang || target_lang.clone()!=old_target_lang {
                    match deepl::translate(text.clone(), target_lang.clone(), source_lang.clone()) {
                        Ok(t) => {
                            *text = t 
                        },
                        Err(err) => {
                            panic!("{}", err)
                        }
                    }
                };          
            });

            ui.add(egui::TextEdit::multiline(text).desired_width(width).desired_rows(4));           
        });

        // frame.set_window_size(egui::vec2(400.0, 300.0));
        frame.set_window_size(ctx.used_size());
    }
}