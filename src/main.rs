use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, TextView, ScrolledWindow};
use gtk4::glib;

fn main() {
    let app = Application::builder().application_id("com.potato.notepad").build();

    app.connect_activate(|app| {

        let window = ApplicationWindow::builder().application(app).title("POTATO Pad").default_width(600).default_height(400).build();
        window.set_resizable(false);
        let text_view = TextView::new();
        let scroller = ScrolledWindow::builder().child(&text_view).build();

        let display = Display::default().expect("nothing's there");
        glib::timeout_add_local(std::time::Duration::from_millis(300),move || {
            let text_view_clone = text_view.clone();
            let clipboard = display.clipboard();
            glib::spawn_future_local(async move {
                if let Ok(Some(text)) = clipboard.read_text_future().await{
                    let buffer = text_view_clone.buffer();
                    buffer.set_text(text.as_str());
                }
                
            });
            
            glib::ControlFlow::Continue
        });

        window.set_child(Some(&scroller));
        window.show();
    });


    app.run();

}
