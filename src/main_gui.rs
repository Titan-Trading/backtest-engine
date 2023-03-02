use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Entry, TextView, TextBuffer, TextTagTable};

fn main() {
    let app = Application::new(Some("com.example"), Default::default());

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("My GTK App");
        window.set_default_size(350, 70);

        let main_box = Box::new(gtk::Orientation::Vertical, 0);
        window.set_child(Some(&main_box));

        let entry = Entry::builder()
            .placeholder_text("Enter text here...")
            .build();
        main_box.pack_start(&entry, false, false, 0);

        let button = Button::builder()
            .label("Submit")
            .build();
        main_box.pack_start(&button, false, false, 0);

        let buffer = TextBuffer::builder()
            .tag_table(&TextTagTable::new())
            .build();
        let output = TextView::builder()
            .buffer(&buffer)
            .build();
        main_box.pack_start(&output, true, true, 0);

        let entry_copy = entry.clone();
        button.connect_clicked(move |_| {
            let entry = entry_copy.clone();
            let input = entry.text().to_string();
            buffer.insert_at_cursor(&format!("You entered: {}\n", input));
            entry.set_text("");
        });

        window.show();
    });

    app.run();
}
