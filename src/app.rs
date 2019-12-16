use crate::file_editor::FileEditor;
use glib::WeakRef;
use gtk::{
    prelude::*, Application, ApplicationWindow, BoxBuilder, Label, Menu, MenuBarBuilder, MenuItem,
    Notebook, NotebookBuilder, Orientation, Paned, SeparatorMenuItem, TreeView, TreeViewBuilder,
    WindowPosition,
};
use log::*;
use std::{cell::RefCell, path::PathBuf, rc::Rc};

///coordinates the application ui and FileEditor objects
pub struct App {
    window: WeakRef<ApplicationWindow>,
    _file_view: TreeView,
    notebook: Notebook,
    current_directory: Option<PathBuf>,
    file_editors: Vec<Rc<FileEditor>>,
}

impl App {
    pub fn run(application: &Application) {
        let window = ApplicationWindow::new(application);
        let window_weak = window.downgrade();

        window.set_title("RJournal");
        window.set_border_width(0);
        window.set_position(WindowPosition::Center);
        window.set_default_size(640, 480);

        let vbox = BoxBuilder::new().homogeneous(false).spacing(0).build();
        vbox.set_orientation(Orientation::Vertical);

        //build the menu bar
        let menu_bar = MenuBarBuilder::new().build();

        let file = MenuItem::new_with_label("File");

        let file_menu = Menu::new();
        let new = MenuItem::new_with_label("New");
        let save = MenuItem::new_with_label("Save");
        let save_as = MenuItem::new_with_label("Save As");
        let open = MenuItem::new_with_label("Open");
        let open_folder = MenuItem::new_with_label("Open Folder");
        let exit = MenuItem::new_with_label("Exit");

        file_menu.add(&new);
        file_menu.add(&open);
        file_menu.add(&open_folder);
        file_menu.add(&SeparatorMenuItem::new());
        file_menu.add(&save);
        file_menu.add(&save_as);
        file_menu.add(&SeparatorMenuItem::new());
        file_menu.add(&exit);
        file.set_submenu(Some(&file_menu));
        menu_bar.add(&file);

        let help = MenuItem::new_with_label("Help");
        menu_bar.add(&help);

        //Build the main content area
        let content_panes = Paned::new(Orientation::Horizontal);

        let file_view = TreeViewBuilder::new()
            .border_width(0)
            .width_request(100)
            .build();
        content_panes.add1(&file_view);

        let notebook = NotebookBuilder::new()
            .scrollable(true)
            .show_tabs(true)
            .enable_popup(false)
            .build();
        //always show the most recently added page
        notebook.connect_page_added(|nb, _child, page| {
            nb.set_current_page(Some(page));
        });
        content_panes.add2(&notebook);

        //Build the bottom bar
        let version_string = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        let info_label = Label::new(Some(&version_string));
        let bottom_bar = gtk::Box::new(Orientation::Horizontal, 0);
        bottom_bar.pack_end(&info_label, false, false, 0);

        //Build the app
        let app = Rc::new(RefCell::new(App {
            window: window_weak,
            _file_view: file_view,
            notebook,
            current_directory: None,
            file_editors: Vec::new(),
        }));

        //Hook up actions to the app
        let app_ref = app.clone();
        exit.connect_activate(move |_| {
            app_ref.borrow().exit();
        });

        let app_ref = app.clone();
        open.connect_activate(move |_| {
            //TODO: try_borrow_mut instead
            app_ref.borrow_mut().open_file();
        });

        //Build and show the window
        vbox.pack_start(&menu_bar, false, false, 0);
        vbox.pack_start(&content_panes, true, true, 0);
        vbox.pack_end(&bottom_bar, false, false, 0);
        window.add(&vbox);

        window.show_all();
    }

    /// Prompt the user for a file and generate a new FileEditor for it.
    fn open_file(&mut self) {
        //Maybe look into gtk::NativeDialogChooser as well.
        let chooser = gtk::FileChooserDialog::with_buttons(
            Some("Open File"),
            None::<&ApplicationWindow>,
            gtk::FileChooserAction::Open,
            &[
                ("_Cancel", gtk::ResponseType::Cancel),
                ("_Open", gtk::ResponseType::Accept),
            ],
        );
        chooser.set_local_only(true);

        //set the FileChoosers directory
        if let Some(dir) = &self.current_directory {
            chooser.set_current_folder(dir);
        } else if let Ok(dir) = std::env::current_dir() {
            chooser.set_current_folder(dir);
        } else {
            debug!("Couldn't set the FileChooserDirectory");
        }

        //Note: Also worth checking out Dialog::connect_response() for a non-blocking version
        let res = chooser.run();
        match res {
            gtk::ResponseType::Accept => {
                if let Some(selection) = chooser.get_file() {
                    self.add_file(selection);
                }
            }
            _ => {}
        }
        chooser.destroy();
    }

    fn add_file(&mut self, file: gio::File) {
        if let Some(fe) = FileEditor::new_from_path(file) {
            let fe_ref = Rc::new(fe);
            FileEditor::build_widget(fe_ref.clone(), &self.notebook);
            self.file_editors.push(fe_ref);
        }
    }

    fn exit(&self) {
        let window_weak = &self.window;
        let window = upgrade_weak!(window_weak);
        window.destroy();
    }
}
