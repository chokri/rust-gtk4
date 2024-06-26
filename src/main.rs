use gtk::{gio, prelude::*, Button};

fn main() {
    // Create a new application
    let application = gtk::Application::builder()
        .application_id("com.github.chokri-gtk4-rust")
        .build();
    application.connect_startup(on_startup);
    application.connect_activate(on_activate);
    application.run();
}

fn on_startup(app: &gtk::Application) {
    let about = gio::ActionEntry::builder("about")
        .activate(|_, _, _| println!("About was pressed"))
        .build();

    let quit = gio::ActionEntry::builder("quit")
        .activate(|app: &gtk::Application, _, _| app.quit())
        .build();

    app.add_action_entries([about, quit]);

    let menubar = {
        let file_menu = {
            let about_menu_item = gio::MenuItem::new(Some("About"), Some("app.about"));
            let quit_menu_item = gio::MenuItem::new(Some("Quit"), Some("app.quit"));

            let file_menu = gio::Menu::new();
            file_menu.append_item(&about_menu_item);
            file_menu.append_item(&quit_menu_item);
            file_menu
        };

        let menubar = gio::Menu::new();
        menubar.append_submenu(Some("File"), &file_menu);

        menubar
    };

    app.set_menubar(Some(&menubar));
}

fn on_activate(application: &gtk::Application) {
    let button = Button::with_label("Open");
    button.connect_clicked(|_| {
        eprint!("Clicked!");
    });
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("GTK4 Rust Demo")
        .default_width(350)
        .default_height(350)
        .show_menubar(true)
        .build();

    window.set_child(Some(&button));

    window.present();
}
