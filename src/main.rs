use gtk4::gdk::Display;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, TextView, ScrolledWindow};
use gtk4::glib;

fn main() {
    let app = Application::builder().application_id("com.potato.notepad").build();
    let last_text = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
    app.connect_activate(|app| {

        let window = ApplicationWindow::builder().application(app).title("POTATO Pad").default_width(600).default_height(400).build();
        window.set_resizable(false);
        let text_view = TextView::new();
        let scroller = ScrolledWindow::builder().child(&text_view).build();

        let display = Display::default().expect("nothing's there");
        let display = Display::default().expect("no display?");
let clipboard = display.clipboard();

let text_view_clone = text_view.clone();
let last_text_clone = last_text.clone();

glib::timeout_add_local(std::time::Duration::from_millis(300), move || {
    let text_view = text_view_clone.clone();
    let clipboard = clipboard.clone();
    let last_text = last_text_clone.clone();

    glib::spawn_future_local(async move {
        if let Ok(Some(text)) = clipboard.read_text_future().await {
            let new_text = text.to_string();

            // check if it's different from the last one
            let mut last = last_text.borrow_mut();
            if *last != new_text {
                // yep it's new, so append it
                let buffer = text_view.buffer();
                let mut end_iter = buffer.end_iter();
                buffer.insert(&mut end_iter, &format!("{}\n", new_text));

                // update memory
                *last = new_text;
            }
        }
    });

    glib::ControlFlow::Continue
});

        window.set_child(Some(&scroller));
        window.show();
    });


    app.run();

}
