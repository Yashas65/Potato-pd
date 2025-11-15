use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, TextView, ScrolledWindow};

fn main() {
    let app = Application::builder().application_id("com.potato.notepad").build();

    app.connect_activate(|app| {

        let window = ApplicationWindow::builder().application(app).title("POTATO Pad").default_width(600).default_height(400).build();
        window.set_resizable(false);
        let text_view = TextView::new();

        let scroller = ScrolledWindow::builder().child(&text_view).build();

        window.set_child(Some(&scroller));
        window.show();
    });

    app.run();

}
