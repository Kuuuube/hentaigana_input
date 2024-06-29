#[derive(serde::Deserialize, serde::Serialize)]
pub struct HentaiganaInputSettings {
    pub dark_mode: bool,
    pub textedit_font_size: f32,
    pub ime_font_size: f32,
    pub ime_sidebar_width: f32,
    pub ime_shortcuts: bool,

    pub textedit_font_size_string: String,
    pub ime_font_size_string: String,
    pub ime_sidebar_width_string: String,
}

impl Default for HentaiganaInputSettings {
    fn default() -> Self {
        Self {
            dark_mode: true,
            textedit_font_size: 50.0,
            ime_font_size: 40.0,
            ime_sidebar_width: 250.0,
            ime_shortcuts: true,

            textedit_font_size_string: "50".to_owned(),
            ime_font_size_string: "40".to_owned(),
            ime_sidebar_width_string: "250".to_owned(),
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct HentaiganaInputGui {
    text: String,
    settings: HentaiganaInputSettings,

    #[serde(skip)]
    ime_text: String,
    #[serde(skip)]
    blocked_keys: Vec<String>,
}

impl Default for HentaiganaInputGui {
    fn default() -> Self {
        Self {
            text: "".to_owned(),
            settings: HentaiganaInputSettings::default(),

            ime_text: "".to_owned(),
            blocked_keys: vec![],
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
        crate::font::set_font_styles(&mut self.settings, ctx);

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

                        egui::Grid::new("hentaigana_selection_grid").show(ui, |ui| {
                            ui.add_sized(
                                ui.available_size(),
                                egui::Label::new("Textarea Font Size:").selectable(false),
                            );
                            let response = ui.add_sized(
                                ui.available_size(),
                                egui::TextEdit::singleline(
                                    &mut self.settings.textedit_font_size_string,
                                ),
                            );
                            if response.changed() {
                                set_font_size(&mut self.settings);
                            }
                            ui.end_row();

                            ui.add(egui::Label::new("IME Font Size:").selectable(false));
                            let response = ui.add_sized(
                                ui.available_size(),
                                egui::TextEdit::singleline(&mut self.settings.ime_font_size_string),
                            );
                            if response.changed() {
                                set_font_size(&mut self.settings);
                            }
                            ui.end_row();

                            ui.add(egui::Label::new("IME Sidebar Width:").selectable(false));
                            let response = ui.add_sized(
                                ui.available_size(),
                                egui::TextEdit::singleline(
                                    &mut self.settings.ime_sidebar_width_string,
                                ),
                            );
                            if response.changed() {
                                set_sidebar_size(&mut self.settings);
                            }
                            ui.end_row();

                            ui.add(egui::Checkbox::new(
                                &mut self.settings.ime_shortcuts,
                                "IME Shortcuts",
                            ));
                            ui.end_row();
                        });

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

        egui::SidePanel::new(egui::panel::Side::Right, "right_sidepanel")
            .min_width(self.settings.ime_sidebar_width)
            .max_width(self.settings.ime_sidebar_width)
            .resizable(false)
            .show(ctx, |ui| {
                egui::Grid::new("hentaigana_selection_grid").show(ui, |ui| {
                    setup_ime_labels(ui, self);
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut textedit_has_focus = false;
            ctx.memory(|memory| {
                textedit_has_focus = memory.has_focus("hentaigana_textedit".into());
            });
            if textedit_has_focus && self.settings.ime_shortcuts {
                filter_events_and_replace(self, ui, self.blocked_keys.clone());
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.text)
                        .lock_focus(true)
                        .font(egui::TextStyle::Name("textedit".into()))
                        .min_size(egui::Vec2 { x: 300.0, y: 300.0 })
                        .id("hentaigana_textedit".into()),
                );
            });
        });
    }
}

fn setup_ime_labels(ui: &mut egui::Ui, hentaigana_input_gui: &mut HentaiganaInputGui) {
    let hentaigana_display =
        crate::hentaigana_dicts::get_hentaigana_display(hentaigana_input_gui.text.clone());

    let mut button_label_width = 50.0;
    if !hentaigana_input_gui.settings.ime_shortcuts {
        button_label_width *= 2.0;
    }
    let ime_text_style = egui::TextStyle::Name("ime".into());
    let mut blocked_keys: Vec<String> = vec![];

    for (left_display, right_display) in hentaigana_display {
        blocked_keys.push(left_display.left.clone());

        if left_display.right.len() > 0 {
            if hentaigana_input_gui.settings.ime_shortcuts {
                ui.add_sized(
                    [button_label_width, 0.0],
                    egui::Label::new(
                        egui::RichText::new(left_display.left.clone())
                            .text_style(ime_text_style.clone()),
                    )
                    .selectable(false),
                );
            }

            let left_selectable_label = ui.add_sized(
                [button_label_width, 0.0],
                egui::SelectableLabel::new(
                    false,
                    egui::RichText::new(left_display.right).text_style(ime_text_style.clone()),
                ),
            );

            if left_selectable_label.clicked() {
                replace_text(hentaigana_input_gui, left_display.left);
            }
        } else {
            //placeholders
            if hentaigana_input_gui.settings.ime_shortcuts {
                ui.add_sized(
                    [button_label_width, 0.0],
                    egui::Label::new(egui::RichText::new("").text_style(ime_text_style.clone()))
                        .selectable(false),
                );
            }

            ui.add_sized(
                [button_label_width, 0.0],
                egui::Label::new(egui::RichText::new("").text_style(ime_text_style.clone()))
                    .selectable(false),
            );
        }

        if right_display.right.len() > 0 {
            blocked_keys.push(right_display.left.clone());

            if hentaigana_input_gui.settings.ime_shortcuts {
                ui.add_sized(
                    [button_label_width, 0.0],
                    egui::Label::new(
                        egui::RichText::new(right_display.left.clone())
                            .text_style(ime_text_style.clone()),
                    )
                    .selectable(false),
                );
            }
            let right_selectable_label = ui.add_sized(
                [button_label_width, 0.0],
                egui::SelectableLabel::new(
                    false,
                    egui::RichText::new(right_display.right).text_style(ime_text_style.clone()),
                ),
            );
            if right_selectable_label.clicked() {
                replace_text(hentaigana_input_gui, right_display.left);
            }
        }

        ui.end_row();
    }
    hentaigana_input_gui.blocked_keys = blocked_keys;
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

fn filter_events_and_replace(
    hentaigana_input_gui: &mut HentaiganaInputGui,
    ui: &mut egui::Ui,
    blocked_keys: Vec<String>,
) {
    ui.input_mut(|i| {
        for event in &i.events {
            match event {
                egui::Event::Text(t) => {
                    let normalized_t =
                        crate::normalize_input::fullwidth_to_halfwidth(t.to_string());
                    if blocked_keys.contains(&normalized_t) {
                        replace_text(hentaigana_input_gui, normalized_t);
                        i.events = vec![];
                        break;
                    }
                }
                _ => {}
            }
        }
    });
}

fn replace_text(hentaigana_input_gui: &mut HentaiganaInputGui, display_left: String) {
    let replace_data = crate::hentaigana_dicts::get_hentaigana_replace(
        hentaigana_input_gui.text.clone(),
        display_left,
    );
    let re = regex::Regex::new(&(replace_data.1 + "$")).unwrap();
    hentaigana_input_gui.text = re
        .replace(&hentaigana_input_gui.text, &replace_data.0)
        .to_string();
}

fn set_font_size(settings: &mut HentaiganaInputSettings) {
    match settings.textedit_font_size_string.parse::<f32>() {
        Ok(ok) => {
            if ok > 0.0 {
                settings.textedit_font_size = ok
            }
        }
        Err(_) => {}
    };

    match settings.ime_font_size_string.parse::<f32>() {
        Ok(ok) => {
            if ok > 0.0 {
                settings.ime_font_size = ok
            }
        }
        Err(_) => {}
    };
}

fn set_sidebar_size(settings: &mut HentaiganaInputSettings) {
    match settings.ime_sidebar_width_string.parse::<f32>() {
        Ok(ok) => {
            if ok > 0.0 {
                settings.ime_sidebar_width = ok
            }
        }
        Err(_) => {}
    };
}
