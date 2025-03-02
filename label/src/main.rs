use fltk::{app::{App, Screen}, frame::Frame, prelude::{GroupExt, WidgetBase, WidgetExt}, window::Window};

const APP_WIDTH : i32 = 320;
const APP_HEIGHT: i32 = 60;

fn main() {
    let screen = Screen::new(0)
        .expect("Failed to find default screen.");

    let app = App::default();
    let mut window = Window::default()
        .with_label("FLTK Label Example")
        .with_size(APP_WIDTH, APP_HEIGHT)
        .with_pos( (screen.w() - APP_WIDTH) / 2, (screen.h() - APP_HEIGHT) / 2);

    let _ = Frame::default_fill()
        .with_label("FLTK Label Example");
    window.end();
    window.show();

    app.run().unwrap();
}
