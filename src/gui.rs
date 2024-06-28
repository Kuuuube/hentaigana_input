#[derive(serde::Deserialize, serde::Serialize)]
pub struct HentaiganaInputSettings {
    dark_mode: bool,
}

impl Default for HentaiganaInputSettings {
    fn default() -> Self {
        Self {
            dark_mode: true,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct HentaiganaInputGui {
    text: String,
    settings: HentaiganaInputSettings,
}

impl Default for HentaiganaInputGui {
    fn default() -> Self {
        Self {
            text: "".to_owned(),
            settings: HentaiganaInputSettings::default(),
        }
    }
}

impl HentaiganaInputGui {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        crate::font::add_babelstonehan_font(cc);

        //restore state
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        return Default::default();
    }
}

impl eframe::App for HentaiganaInputGui {
    //save state
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        set_theme(ctx, self.settings.dark_mode);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });

                    ui.menu_button("Edit", |ui| {
                        if ui.button("Clear text").clicked() {
                            self.text = "".to_owned();
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Settings", |ui| {
                        light_dark_buttons(self, ui);

                        if ui.button("Reset").clicked() {
                            self.settings = HentaiganaInputSettings::default();
                            ui.close_menu();
                        }
                    });
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    unselectable_warn_if_debug_build(ui);
                });
            });
        });

        egui::SidePanel::new(egui::panel::Side::Right, "right_sidepanel").min_width(200.0).resizable(false).show(ctx, |ui| {
            egui::Grid::new("hentaigana_selection_grid").show(ui, |ui| {
                let button_label_width = 100.0;
                ui.add_sized([button_label_width, 0.0], egui::SelectableLabel::new(false, "1"));
                ui.add_sized([button_label_width, 0.0], egui::SelectableLabel::new(false, "!"));
                ui.end_row();

                ui.add_sized([button_label_width, 0.0], egui::SelectableLabel::new(false, "2"));
                ui.add_sized([button_label_width, 0.0], egui::SelectableLabel::new(false, "@"));
                ui.end_row();
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let blocked_keys = vec!["!".to_owned(), "@".to_owned()];
            filter_events(ui, blocked_keys);

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.text).lock_focus(true),
                );
            });
        });
    }
}

fn unselectable_warn_if_debug_build(ui: &mut egui::Ui) {
    if cfg!(debug_assertions) {
        ui.add(
            egui::Label::new(
                egui::RichText::new("⚠ Debug build ⚠")
                    .small()
                    .color(ui.visuals().warn_fg_color),
            )
            .selectable(false),
        );
    }
}

fn light_dark_buttons(hentaigana_input_gui: &mut HentaiganaInputGui, ui: &mut egui::Ui) {
    let mut visuals = ui.ctx().style().visuals.clone();
    visuals.light_dark_radio_buttons(ui);
    hentaigana_input_gui.settings.dark_mode = visuals.dark_mode;
    set_theme(ui.ctx(), visuals.dark_mode);
}

fn set_theme(ctx: &egui::Context, dark_mode: bool) {
    if dark_mode {
        ctx.set_visuals(egui::Visuals::dark());
    } else {
        ctx.set_visuals(egui::Visuals::light());
    }
}

fn filter_events(ui: &mut egui::Ui, blocked_keys: Vec<String>) {
    ui.input_mut(|i| {
        for event in &i.events {
            match event {
                egui::Event::Text(t) => {
                    if blocked_keys.contains(t) {
                        i.events = vec![];
                        break;
                    }
                }
                _ => {}
            }
        }
    });
}
