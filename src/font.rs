use eframe::egui::{self, FontDefinitions, FontFamily, TextStyle};
use egui::{FontData, FontId};
use FontFamily::{Monospace, Proportional};

use crate::cfg::SETTINGS;

pub fn install_fonts(egui_ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "LXGWWenKai-Regular".to_owned(),
        FontData::from_static(include_bytes!("../res/LXGWWenKai-Regular.ttf")),
    );
    fonts
        .families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "LXGWWenKai-Regular".to_owned());
    fonts
        .families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "LXGWWenKai-Regular".to_owned());

    egui_ctx.set_fonts(fonts);

    let font_size_plus = {
        let settings = SETTINGS.lock().unwrap();
        settings.get_float("window.font_size_plus").unwrap_or(0.0) as f32
    };

    let mut style = (*egui_ctx.style()).clone();
    style.text_styles = [
        (
            TextStyle::Heading,
            FontId::new(28.0 + font_size_plus, Proportional),
        ),
        (
            TextStyle::Body,
            FontId::new(20.0 + font_size_plus, Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(18.0 + font_size_plus, Monospace),
        ),
        (
            TextStyle::Button,
            FontId::new(20.0 + font_size_plus, Proportional),
        ),
        (
            TextStyle::Small,
            FontId::new(18.0 + font_size_plus, Proportional),
        ),
    ]
    .into();

    egui_ctx.set_style(style);
}
