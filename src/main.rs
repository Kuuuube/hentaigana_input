#![windows_subsystem = "windows"]

use gtk4 as gtk;
use gtk::{prelude::*, glib, Application, ApplicationWindow, TextView, Box, Label, EventControllerKey};
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

    labels.set_markup("<span font-family='BabelStone Han' size='30000'>hentaigana!</span>");

    let labels_clone = labels.clone();
    let textbox_clone = textbox.clone();

    let controller = EventControllerKey::new();
    controller.connect_key_released(move |controller, key, keycode, state| {
        let buffer = textbox_clone.buffer();
        let current_text = buffer.text(&buffer.start_iter(), &buffer.end_iter(), true).to_string();
        let re = Regex::new(r"ku$").unwrap();

        labels_clone.set_markup(&format!("{}{}{}", "<span font-family='BabelStone Han' size='30000'>", hentaigana_dicts::get_hentaigana_display(current_text, key.name().unwrap().to_string()), "</span>"));
/*         if re.is_match(&current_text) && keycode == 49 {
            labels_clone.set_markup("<span font-family='BabelStone Han' size='30000'>1. 𛀫\n2. 𛀬\n3. 𛀭\n4. 𛀮\n5. 𛀯\n6. 𛀰\n7. 𛀱</span>");
            textbox_clone.buffer().set_text(&re.replace(&current_text, "𛀱"));
            return gtk4::Inhibit(true);
        } */
        //gtk4::Inhibit(false)
    });

    textbox.add_controller(controller);

    let gtkbox = Box::new(gtk::Orientation::Horizontal, 12);

    gtkbox.set_margin_top(12);
    gtkbox.set_margin_bottom(12);
    gtkbox.set_margin_start(12);
    gtkbox.set_margin_end(12);

    let leftbox = Box::new(gtk::Orientation::Vertical, 12);
    leftbox.append(&textbox);
    gtkbox.append(&leftbox);

    let rightbox = Box::new(gtk::Orientation::Vertical, 12);
    rightbox.append(&labels);
    gtkbox.append(&rightbox);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("変体がな")
        .child(&gtkbox)
        .build();

    window.set_default_size(500, 400);

    labels.set_xalign(1.0);

    window.present();
}

fn load_css() {
    let display = Display::default().expect("Could not get default display.");
    let provider = gtk::CssProvider::new();
    let priority = gtk::STYLE_PROVIDER_PRIORITY_APPLICATION;

    provider.load_from_data(include_str!("style.css"));
    
    gtk::style_context_add_provider_for_display(&display, &provider, priority);
}