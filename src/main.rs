#![windows_subsystem = "windows"]

use egui;

mod font;
mod gui;
mod hentaigana_dicts;
mod normalize_input;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([650.0, 725.0])
            .with_min_inner_size([650.0, 725.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Hentaigana Input",
        native_options,
        Box::new(|cc| Box::new(gui::HentaiganaInputGui::new(cc))),
    )
}
