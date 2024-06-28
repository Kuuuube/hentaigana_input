#![windows_subsystem = "windows"]

use egui;

mod gui;
mod hentaigana_dicts;
mod normalize_input;

fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
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
