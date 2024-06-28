pub fn set_font_styles(settings: &mut crate::gui::HentaiganaInputSettings, ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles.insert(egui::TextStyle::Name("textedit".into()), egui::FontId::new(settings.textedit_font_size, egui::FontFamily::Proportional));
    style.text_styles.insert(egui::TextStyle::Name("ime".into()), egui::FontId::new(settings.ime_font_size, egui::FontFamily::Proportional));
    ctx.set_style(style);
}

pub fn add_babelstonehan_font(cc: &eframe::CreationContext<'_>) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "BabelStoneHan".into(),
        egui::FontData::from_owned(
            get_babelstonehan().expect("Could not load font"),
        ),
    );
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .push("BabelStoneHan".into());
    cc.egui_ctx.set_fonts(fonts);
}

fn get_babelstonehan() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let font = include_bytes!("../assets/BabelStoneHan.ttf.zst");
    return Ok(decode_zstd(font)?);
}

fn decode_zstd(data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let bound = zstd_safe::decompress_bound(&data).expect("zstd_safe::decompress_bound failed");
    let mut decompressed: Vec<u8> = Vec::with_capacity(bound.try_into()?);
    zstd_safe::decompress(&mut decompressed, &data).expect("zstd_safe::decompress failed");
    return Ok(decompressed);
}
