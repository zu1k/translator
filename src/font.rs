use eframe::egui::{self, FontDefinitions, FontFamily, TextStyle};
use std::collections::BTreeMap;

pub fn install_fonts(egui_ctx: &egui::CtxRef){
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "LXGWWenKai-Regular".to_owned(),
        std::borrow::Cow::Borrowed(include_bytes!("../res/LXGWWenKai-Regular.ttf")),
    );
    fonts.fonts_for_family.get_mut(&FontFamily::Monospace).unwrap()
        .insert(0, "LXGWWenKai-Regular".to_owned());
    fonts.fonts_for_family.get_mut(&FontFamily::Proportional).unwrap()
        .insert(0, "LXGWWenKai-Regular".to_owned());

    let mut family_and_size = BTreeMap::new();
    family_and_size.insert(TextStyle::Small, (FontFamily::Proportional, 18.0));
    family_and_size.insert(TextStyle::Body, (FontFamily::Proportional, 20.0));
    family_and_size.insert(TextStyle::Button, (FontFamily::Proportional, 20.0));
    family_and_size.insert(TextStyle::Heading, (FontFamily::Proportional, 28.0));
    family_and_size.insert(TextStyle::Monospace, (FontFamily::Monospace, 18.0));
    fonts.family_and_size = family_and_size;

    egui_ctx.set_fonts(fonts);
}