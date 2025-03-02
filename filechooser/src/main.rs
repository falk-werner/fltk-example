use fltk::{app, button::Button, dialog::{FileChooser, FileChooserType}, prelude::{GroupExt, WidgetBase, WidgetExt}, window::Window};

const APP_WIDTH: i32 = 320;
const APP_HEIGHT: i32 = 60;

fn main() {
    let screen = app::Screen::new(0)
        .expect("Failed to find default screen.");

    let app = app::App::default();
    let mut window = Window::default()
        .with_label("FLTK FileChooser Example")
        .with_size(APP_WIDTH, APP_HEIGHT)
        .with_pos((screen.w() - APP_WIDTH) / 2,  (screen.h() - APP_HEIGHT) / 2);

    let mut button = Button::default_fill()
        .with_label("Select File");
    button.set_callback(move |_| {
        let  mut filechooser = FileChooser::new(".", "*.*", FileChooserType::Single, "Select File");
        filechooser.set_preview(false);
        filechooser.show();
        while filechooser.shown() {
            app::wait();
        }
        if let Some(filename) = filechooser.value(1) {
            println!("selected file: {}", filename);
        }
    });

    window.end();
    window.show();

    app.run().unwrap();
}
