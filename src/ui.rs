use crate::font;

use eframe::{egui, epi};

pub struct MyApp {
    text: String
}

impl MyApp {
    pub fn new(text: String) -> Self {
        Self {
            text
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
        let Self { text } = self;

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
            ui.add(egui::TextEdit::multiline(text).desired_width(width).desired_rows(4));           
        });

        // frame.set_window_size(egui::vec2(400.0, 300.0));
        frame.set_window_size(ctx.used_size());
    }
}