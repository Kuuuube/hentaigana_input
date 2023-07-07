#![windows_subsystem = "windows"]

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

    let textbox = TextView::builder().build();

    textbox.set_size_request(476, 388);
    textbox.add_css_class("textview");
    textbox.set_wrap_mode(gtk::WrapMode::Char);

    let labels = Label::builder().build();

    labels.set_markup("<span font-family='BabelStone Han' size='29000'>変\n体\nが\nな\n！</span>");

    let labels_clone = labels.clone();
    let textbox_clone = textbox.clone();

    let release_controller = EventControllerKey::new();
    release_controller.connect_key_released(move |_release_controller, _key, _keycode, _state| {
        let buffer = textbox_clone.buffer();
        let current_text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true).to_string();

        labels_clone.set_markup(&format!("{}{}{}", "<span font-family='BabelStone Han' size='29000'>", hentaigana_dicts::get_hentaigana_display(current_text.clone()), "</span>"));
    });
    textbox.add_controller(release_controller);

    let labels_clone = labels.clone();
    let textbox_clone = textbox.clone();

    let press_controller = EventControllerKey::new();
    press_controller.connect_key_pressed(move |_press_controller, key, _keycode, _state| {
        let keyname = match key.name().unwrap().to_string().as_str() {
            "equal" => "=".to_owned(),
            "minus" => "-".to_owned(),
            _ => key.name().unwrap().to_string()
        };

        let accepted_keycodes: Vec<String> = vec!["1".to_string(), "2".to_string(), "3".to_string(), "4".to_string(), "5".to_string(), "6".to_string(), "7".to_string(), "8".to_string(), "9".to_string(), "0".to_string(), "-".to_string(), "=".to_string()];

        if accepted_keycodes.contains(&keyname) {
            let buffer = textbox_clone.buffer();
            let current_text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true).to_string();

            let find_hentaigana = hentaigana_dicts::get_hentaigana_replace(current_text.clone(), keyname);

            let re = Regex::new(&(find_hentaigana.1 + "$")).unwrap();
            textbox_clone.buffer().set_text(&re.replace(&current_text, find_hentaigana.0));
            if re.is_match(&current_text) {
                labels_clone.set_markup("");
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

    let rightscrollbox = ScrolledWindow::builder()
    .child(&labels)
    .build();

    let rightbox = Box::new(gtk::Orientation::Vertical, 12);
    rightbox.append(&rightscrollbox);
    gtkbox.append(&rightbox);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("変体がな")
        .child(&gtkbox)
        .build();


    let window_size = (800, 600);
    window.set_default_size(window_size.0, window_size.1);

    leftscrollbox.set_size_request((window_size.0 as f64 * 0.8) as i32, window_size.1);
    rightscrollbox.set_size_request((window_size.0 as f64 * 0.2) as i32, window_size.1);

    window.present();
}

fn load_css() {
    let display = Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    provider.load_from_data(include_str!("style.css"));
    
    gtk::style_context_add_provider_for_display(&display, &provider, priority);
}