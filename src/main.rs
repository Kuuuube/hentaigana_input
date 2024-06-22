#![windows_subsystem = "windows"]

use std::fs::File;
use std::io::prelude::*;

use gtk4 as gtk;
use gtk::{prelude::*, glib, Application, ApplicationWindow, TextView, Box, Label, EventControllerKey, ScrolledWindow, HeaderBar, Button};
use gtk::gdk::Display;

use regex::Regex;

mod hentaigana_dicts;
mod normalize_input;

const APP_ID: &str = "org.gtk_rs.Hentaigana_Input";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let gtk_settings = gtk::Settings::default().unwrap();
    gtk::Settings::set_gtk_application_prefer_dark_theme(&gtk_settings, true);

    load_css();

    let textbox = TextView::builder()
        .name("textbox")
        .build();

    textbox.set_size_request(476, 388);
    textbox.set_wrap_mode(gtk::WrapMode::Char);

    let primarylabels = Label::builder()
        .name("primarystartuplabel")
        .build();

    primarylabels.set_label("𛂸\n𛄞\n𛁡\n𛀇\n𛀡゙\n𛁾");

    let secondarylabels = Label::builder()
        .name("secondarystartuplabel")
        .build();

    secondarylabels.set_label("𛂍\n𛃤\n𛀊\n𛃴\n𛃪\n𛀬");

    let primarylabels_clone = primarylabels.clone();
    let secondarylabels_clone = secondarylabels.clone();
    let textbox_clone = textbox.clone();

    let release_controller = EventControllerKey::new();
    release_controller.connect_key_released(move |_release_controller, _key, _keycode, _state| {
        let buffer = textbox_clone.buffer();
        let current_text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true).to_string();

        let hentaigana_display = hentaigana_dicts::get_hentaigana_display(normalize_input::fullwidth_to_halfwidth(current_text.clone()));

        primarylabels_clone.set_label(&hentaigana_display.0);
        primarylabels_clone.set_yalign(0.0);
        primarylabels_clone.set_widget_name("primarylabels");

        secondarylabels_clone.set_label(&hentaigana_display.1);
        secondarylabels_clone.set_yalign(0.0);
        secondarylabels_clone.set_widget_name("secondarylabels");
    });
    textbox.add_controller(release_controller);

    let primarylabels_clone = primarylabels.clone();
    let secondarylabels_clone = secondarylabels.clone();
    let textbox_clone = textbox.clone();

    let press_controller = EventControllerKey::new();
    press_controller.connect_key_pressed(move |_press_controller, key, _keycode, _state| {
        let keyname = match key.to_unicode() {
            Some(some) => some.to_string(),
            None => "".to_string(),
        };

        let accepted_keycodes: Vec<&str> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "+"];

        if accepted_keycodes.contains(&keyname.as_str()) {
            let buffer = textbox_clone.buffer();
            let current_text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true).to_string();

            let find_hentaigana = hentaigana_dicts::get_hentaigana_replace(normalize_input::fullwidth_to_halfwidth(current_text.clone()), keyname);

            let re = Regex::new(&(find_hentaigana.1.clone() + "$")).unwrap();
            textbox_clone.buffer().set_text(&re.replace(&current_text, find_hentaigana.0.clone()));
            if re.is_match(&current_text) {
                let new_buffer = textbox_clone.buffer();
                let new_text = new_buffer.text(&new_buffer.start_iter(), &new_buffer.end_iter(), true).to_string();
                let hentaigana_display = hentaigana_dicts::get_hentaigana_display(normalize_input::fullwidth_to_halfwidth(new_text.clone()));

                primarylabels_clone.set_label(&hentaigana_display.0);
                primarylabels_clone.set_yalign(0.0);
                primarylabels_clone.set_widget_name("primarylabels");

                secondarylabels_clone.set_label(&hentaigana_display.1);
                secondarylabels_clone.set_yalign(0.0);
                secondarylabels_clone.set_widget_name("secondarylabels");
                return glib::Propagation::Stop;
            }
        }
        glib::Propagation::Proceed
    });
    textbox.add_controller(press_controller);

    let dark_light_button = Button::builder()
        .label("Light/Dark Theme")
        .build();

    let gtk_settings_clone = gtk_settings.clone();
    dark_light_button.connect_clicked(move |_dark_light_button| {
        if gtk::Settings::is_gtk_application_prefer_dark_theme(&gtk_settings_clone) {
            gtk::Settings::set_gtk_application_prefer_dark_theme(&gtk_settings_clone, false);
        } else {
            gtk::Settings::set_gtk_application_prefer_dark_theme(&gtk_settings_clone, true);
        }
    });

    let title_label = Label::builder()
        .name("titlelabel")
        .label("𛂸𛄞𛁡𛀇𛀡゙𛁾 𛂍𛃤𛀊𛃴𛃪𛀬")
        .build();

    let headerbar = HeaderBar::builder()
        .title_widget(&title_label)
        .build();

    headerbar.pack_start(&dark_light_button);

    let contentbox = Box::new(gtk::Orientation::Horizontal, 12);

    contentbox.set_margin_top(12);
    contentbox.set_margin_bottom(12);
    contentbox.set_margin_start(12);
    contentbox.set_margin_end(12);

    let leftscrollbox = ScrolledWindow::builder()
        .child(&textbox)
        .build();

    let leftbox = Box::new(gtk::Orientation::Vertical, 12);
    leftbox.append(&leftscrollbox);
    contentbox.append(&leftbox);

    let primaryrightscrollbox = ScrolledWindow::builder()
        .child(&primarylabels)
        .build();

    let secondaryrightscrollbox = ScrolledWindow::builder()
        .child(&secondarylabels)
        .build();

    let rightbox = Box::new(gtk::Orientation::Horizontal, 12);
    rightbox.append(&primaryrightscrollbox);
    rightbox.append(&secondaryrightscrollbox);
    contentbox.append(&rightbox);

    let window = ApplicationWindow::builder()
        .application(app)
        .child(&contentbox)
        .build();

    let window_size = (960, 600);
    window.set_default_size(window_size.0, window_size.1);

    leftscrollbox.set_size_request((window_size.0 as f64 * 0.6667) as i32, window_size.1);
    primaryrightscrollbox.set_size_request((window_size.0 as f64 * 0.1667) as i32, window_size.1);
    secondaryrightscrollbox.set_size_request((window_size.0 as f64 * 0.1667) as i32, window_size.1);

    window.set_titlebar(Some(&headerbar));

    window.present();
}

fn load_css() {
    let display = Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    let mut css_data = String::new();
    match File::open("style.css") {
        Ok(mut ok) => {ok.read_to_string(&mut css_data).unwrap();},
        Err(_) => {css_data = include_str!("style.css").to_string()}
    }

    provider.load_from_data(&css_data);
    
    gtk::style_context_add_provider_for_display(&display, &provider, priority);
}