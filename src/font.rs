use crate::SETTINGS;
use eframe::egui::{self, FontDefinitions, FontFamily, TextStyle};
use std::collections::BTreeMap;

pub fn install_fonts(egui_ctx: &egui::CtxRef) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "LXGWWenKai-Regular".to_owned(),
        std::borrow::Cow::Borrowed(include_bytes!("../res/LXGWWenKai-Regular.ttf")),
    );
    fonts
        .fonts_for_family
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "LXGWWenKai-Regular".to_owned());
    fonts
        .fonts_for_family
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "LXGWWenKai-Regular".to_owned());

    let font_size_plus = {
        let settings = SETTINGS.read().unwrap();
        settings.get_float("window.font_size_plus").unwrap_or(0.0) as f32
    };

    let mut family_and_size = BTreeMap::new();
    family_and_size.insert(
        TextStyle::Small,
        (FontFamily::Proportional, 18.0 + font_size_plus),
    );
    family_and_size.insert(
        TextStyle::Body,
        (FontFamily::Proportional, 20.0 + font_size_plus),
    );
    family_and_size.insert(
        TextStyle::Button,
        (FontFamily::Proportional, 20.0 + font_size_plus),
    );
    family_and_size.insert(
        TextStyle::Heading,
        (FontFamily::Proportional, 28.0 + font_size_plus),
    );
    family_and_size.insert(
        TextStyle::Monospace,
        (FontFamily::Monospace, 18.0 + font_size_plus),
    );
    fonts.family_and_size = family_and_size;

    egui_ctx.set_fonts(fonts);
}
