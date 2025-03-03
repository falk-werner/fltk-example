use fltk::{app::{App, Screen}, enums::CallbackTrigger, group::Flex, input::Input, prelude::{GroupExt, InputExt, WidgetBase, WidgetExt}, window::Window};

const APP_WIDTH: i32 = 320;
const APP_HEIGHT: i32 = 30;

fn main() {
    let screen = Screen::new(0)
        .expect("Failed to find default screen");

    let app = App::default();
    let mut window = Window::default()
        .with_label("FLTK Input Example")
        .with_size(APP_WIDTH, APP_HEIGHT)
        .with_pos( (screen.w() - APP_WIDTH) / 2, (screen.h() - APP_HEIGHT) / 2);
    let col = Flex::default_fill().column();

    let mut input = Input::default();
    input.set_trigger(CallbackTrigger::Changed);
    input.set_callback(|inp| {
        println!("{}", inp.value());
    });

    window.resizable(&col);
    col.end();

    window.end();
    window.show();

    app.run().unwrap();
}
