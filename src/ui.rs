use gtk::prelude::*;
use gtk::*;
use log::*;

use crate::actions;

fn build_top_menu(window: &ApplicationWindow, text_view: &TextView) -> MenuBar {
    let window_weak = window.downgrade();
    let text_view = text_view.clone();

    let menu_bar = MenuBarBuilder::new().build();

    let file = MenuItem::new_with_label("File");

    let file_menu = Menu::new();
    let new = MenuItem::new_with_label("New");
    let save = MenuItem::new_with_label("Save");
    let load = MenuItem::new_with_label("Load");
    let exit = MenuItem::new_with_label("Exit");

    save.connect_activate(move |_| {
        if let Err(err) = actions::save_file(&text_view) {
            error!("Unable to save file: {}", err);
        }
    });
    exit.connect_activate(move |_| {
        let window = upgrade_weak!(window_weak);
        window.destroy();
    });

    file_menu.add(&new);
    file_menu.add(&save);
    file_menu.add(&load);
    file_menu.add(&exit);
    file.set_submenu(Some(&file_menu));
    menu_bar.add(&file);

    let help = MenuItem::new_with_label("Help");
    menu_bar.add(&help);

    menu_bar
}

pub fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("First GTK+ Program");
    window.set_border_width(0);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(350, 70);

    let vbox = gtk::BoxBuilder::new()
        .homogeneous(false)
        .spacing(0)
        .hexpand(false)
        .build();
    vbox.set_orientation(gtk::Orientation::Vertical);

    let text_scroll = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
    let text_view = gtk::TextViewBuilder::new().accepts_tab(true).build();
    text_scroll.add(&text_view);

    let menu = build_top_menu(&window, &text_view);

    vbox.pack_start(&menu, false, false, 0);
    vbox.pack_end(&text_scroll, true, true, 0);
    window.add(&vbox);

    window.show_all();
}
