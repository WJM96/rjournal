//use std::path::P
use crate::util::*;
use gio::prelude::*;
use gtk::{prelude::*, TextTagTable};
use log::*;
use sourceview::{prelude::*, Buffer, LanguageManager};
use std::rc::Rc;

/// Manages the view and buffer for a given file.
pub struct FileEditor {
    text_buffer: Buffer,
    file: gio::File,
}

impl FileEditor {
    pub fn get_file_name(&self) -> String {
        if let Some(path) = self.file.get_basename() {
            path.to_string_lossy().to_string()
        } else {
            String::from("New File")
        }
    }

    ///Set the language for a buffer
    fn update_language(&self) {
        if let Some(path) = self.file.get_basename() {
            let file_name = path.to_string_lossy();
            if let Some(lang_manager) = LanguageManager::get_default() {
                if let Some(language) = lang_manager.guess_language(Some(&file_name), None) {
                    self.text_buffer.set_language(Some(&language));
                } else {
                    info!("Couldn't detect language for {:?}", file_name)
                }
            } else {
                error!("Couldn't find the default language manager");
            }
        }
    }

    //maybe use a result instead of option
    pub fn new_from_path(file: gio::File) -> Option<Self> {
        let text_buffer = Buffer::new(None::<&TextTagTable>);

        let read = load_file(&file);
        match read {
            Ok(contents) => {
                text_buffer.set_text(&contents);
                let new_fe = Self { text_buffer, file };
                new_fe.update_language();
                Some(new_fe)
            }
            Err(err) => {
                error!("Couldn't load file {}", err);
                None
            }
        }
    }

    /// Builds a widget and adds it to a gtk::Notebook.
    // This uses Rc<Self> instead of &self so that actions
    // may be hooked up to the widget that capture the FileEditor object.
    pub fn build_widget(fe: Rc<Self>, notebook: &gtk::Notebook) {
        //build the text view
        let scrolled_window = gtk::ScrolledWindow::new(gtk::NONE_ADJUSTMENT, gtk::NONE_ADJUSTMENT);
        let text_view = sourceview::View::new_with_buffer(&fe.text_buffer);
        text_view.set_show_line_numbers(true);
        scrolled_window.add(&text_view);
        //gtk wont show tabs with hidden
        scrolled_window.show_all();
        notebook.add(&scrolled_window);

        //configure the tab
        let tab_title = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let tab_label = gtk::Label::new(Some(&fe.get_file_name()));
        const EXIT_BUTTON_SIZE: i32 = 16;
        //TODO: add an actual "x". right now it just shows a janky circle
        //when you hover over the button, or nothing depending on your gtk
        //theme.
        let tab_exit_button = gtk::ButtonBuilder::new()
            .relief(gtk::ReliefStyle::None)
            .width_request(EXIT_BUTTON_SIZE)
            .height_request(EXIT_BUTTON_SIZE)
            .build();
        tab_title.pack_start(&tab_label, true, true, 0);
        tab_title.pack_end(&tab_exit_button, true, true, 5);
        tab_title.show_all();

        notebook.set_tab_label(&scrolled_window, Some(&tab_title));
        notebook.set_tab_reorderable(&scrolled_window, true);

        //hook up the close button.
        tab_exit_button.connect_clicked(move |_| {
            //TODO: check that file hasn't been modified since last save
            fe.close();
            scrolled_window.destroy();
        });
    }

    pub fn close(&self) {
        //TODO:
        debug!("FileEditor Close Event");
    }
}
