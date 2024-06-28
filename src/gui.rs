#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct HentaiganaInputGui {
    text: String,
    dark_mode: bool,
    //#[serde(skip)] to opt-out of serialization of a field
}

impl Default for HentaiganaInputGui {
    fn default() -> Self {
        Self {
            text: "".to_owned(),
            dark_mode: true,
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
        set_theme(ctx, self.dark_mode);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.menu_button("Menu", |ui| {
                        light_dark_buttons(self, ui);

                        if ui.button("Clear text").clicked() {
                            self.text = "".to_owned();
                            ui.close_menu();
                        }

                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    unselectable_warn_if_debug_build(ui);
                });
            });
        });

        egui::SidePanel::right("right_sidepanel").show(ctx, |ui| {
            for i in 1..=3 {
                let sidebar_item = ui.selectable_label(false, format!("Button {}", i));
                if sidebar_item.clicked() {
                    println!("clicked {}", i);
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.input_mut(|i| {
                let blocked_keys = vec!["!".to_owned(), "@".to_owned()];
                i.events = i
                    .events
                    .iter()
                    .filter(|x| match x {
                        egui::Event::Text(t) => {
                            if blocked_keys.contains(t) {
                                return false;
                            } else {
                                return true;
                            }
                        }
                        _ => return true,
                    })
                    .map(|x| x.to_owned())
                    .collect();
            });

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
    hentaigana_input_gui.dark_mode = visuals.dark_mode;
    set_theme(ui.ctx(), visuals.dark_mode);
}

fn set_theme(ctx: &egui::Context, dark_mode: bool) {
    if dark_mode {
        ctx.set_visuals(egui::Visuals::dark());
    } else {
        ctx.set_visuals(egui::Visuals::light());
    }
}
