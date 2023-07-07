#![windows_subsystem = "windows"]

use std::fs::File;
use std::io::prelude::*;

use gtk4 as gtk;
use gtk::{prelude::*, glib, Application, ApplicationWindow, TextView, Box, Label, EventControllerKey, ScrolledWindow};
use gtk::gdk::Display;

use regex::Regex;

mod hentaigana_dicts;

const APP_ID: &str = "org.gtk_rs.Hentaigana_Input";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    load_css();

    let textbox = TextView::builder()
        .name("textbox")
        .build();

    textbox.set_size_request(476, 388);
    textbox.set_wrap_mode(gtk::WrapMode::Char);

    let primarylabels = Label::builder()
        .name("primarylabels")
        .build();

    primarylabels.set_label("変\n体\nが\nな\n！");

    let secondarylabels = Label::builder()
        .name("secondarylabels")
        .build();

    let primarylabels_clone = primarylabels.clone();
    let secondarylabels_clone = secondarylabels.clone();
    let textbox_clone = textbox.clone();

    let release_controller = EventControllerKey::new();
    release_controller.connect_key_released(move |_release_controller, _key, _keycode, _state| {
        let buffer = textbox_clone.buffer();
        let current_text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true).to_string();

        let hentaigana_display = hentaigana_dicts::get_hentaigana_display(current_text.clone());

        primarylabels_clone.set_markup(&hentaigana_display.0);
        primarylabels_clone.set_yalign(0.0);
        secondarylabels_clone.set_markup(&hentaigana_display.1);
        secondarylabels_clone.set_yalign(0.0);
    });
    textbox.add_controller(release_controller);

    let primarylabels_clone = primarylabels.clone();
    let secondarylabels_clone = secondarylabels.clone();
    let textbox_clone = textbox.clone();

    let press_controller = EventControllerKey::new();
    press_controller.connect_key_pressed(move |_press_controller, key, _keycode, _state| {
        let keyname = match key.name().unwrap().to_string().as_str() {
            "equal" => "=".to_owned(),
            "minus" => "-".to_owned(),
            "exclam" => "!".to_owned(),
            "at" => "@".to_owned(),
            "numbersign" => "#".to_owned(),
            "dollar" => "$".to_owned(),
            "percent" => "%".to_owned(),
            "asciicircum" => "^".to_owned(),
            "ampersand" => "&".to_owned(),
            "asterisk" => "*".to_owned(),
            "parenleft" => "(".to_owned(),
            "parenright" => ")".to_owned(),
            "underscore" => "_".to_owned(),
            "plus" => "+".to_owned(),
            _ => key.name().unwrap().to_string()
        };

        let accepted_keycodes: Vec<&str> = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "!", "@", "#", "$", "%", "^", "&", "*", "(", ")", "_", "+"];

        if accepted_keycodes.contains(&keyname.as_str()) {
            let buffer = textbox_clone.buffer();
            let current_text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true).to_string();

            let find_hentaigana = hentaigana_dicts::get_hentaigana_replace(current_text.clone(), keyname);

            let re = Regex::new(&(find_hentaigana.1 + "$")).unwrap();
            textbox_clone.buffer().set_text(&re.replace(&current_text, find_hentaigana.0));
            if re.is_match(&current_text) {
                primarylabels_clone.set_markup("");
                secondarylabels_clone.set_markup("");
                return gtk4::Inhibit(true);
            }
        }
        gtk4::Inhibit(false)
    });
    textbox.add_controller(press_controller);

    let gtkbox = Box::new(gtk::Orientation::Horizontal, 12);

    gtkbox.set_margin_top(12);
    gtkbox.set_margin_bottom(12);
    gtkbox.set_margin_start(12);
    gtkbox.set_margin_end(12);

    let leftscrollbox = ScrolledWindow::builder()
    .child(&textbox)
    .build();

    let leftbox = Box::new(gtk::Orientation::Vertical, 12);
    leftbox.append(&leftscrollbox);
    gtkbox.append(&leftbox);

    let primaryrightscrollbox = ScrolledWindow::builder()
    .child(&primarylabels)
    .build();

    let secondaryrightscrollbox = ScrolledWindow::builder()
    .child(&secondarylabels)
    .build();

    let rightbox = Box::new(gtk::Orientation::Horizontal, 12);
    rightbox.append(&primaryrightscrollbox);
    rightbox.append(&secondaryrightscrollbox);
    gtkbox.append(&rightbox);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("変体がな")
        .child(&gtkbox)
        .build();


    let window_size = (960, 600);
    window.set_default_size(window_size.0, window_size.1);

    leftscrollbox.set_size_request((window_size.0 as f64 * 0.6667) as i32, window_size.1);
    primaryrightscrollbox.set_size_request((window_size.0 as f64 * 0.1667) as i32, window_size.1);
    secondaryrightscrollbox.set_size_request((window_size.0 as f64 * 0.1667) as i32, window_size.1);

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