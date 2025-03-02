use std::{cell::RefCell, rc::Rc};

use fltk::{app::{App, Screen}, button::Button, frame::Frame, group::Flex, prelude::{GroupExt, WidgetBase, WidgetExt}, window::Window};

const APP_WIDTH: i32 = 320;
const APP_HEIGHT: i32 = 240;

struct AppState {
    count: i32,
    display: Frame
}

impl AppState {
    fn inc(&mut self) {
        self.count += 1;
        self.display.set_label(format!("{}", self.count).as_str());
    }

    fn dec(&mut self) {
        self.count -= 1;
        self.display.set_label(format!("{}", self.count).as_str());
    }
}

fn main() {
    let screen = Screen::new(0)
        .expect("Failed to find default screen.");


    let app = App::default();
    let mut window = Window::default()
        .with_label("FLTK Counter Example")
        .with_size(APP_WIDTH, APP_HEIGHT)
        .with_pos( (screen.w() - APP_WIDTH) / 2, (screen.h() - APP_HEIGHT) / 2);

    let col = Flex::default_fill().column();


    let display = Frame::default().with_label("0");
    let state = Rc::new(RefCell::new(AppState {
        count: 0,
        display,
    }));

    let mut inc_button = Button::default().with_label("+");
    inc_button.set_callback( {
        let state = state.clone();        
        move |_| {
        let mut state = state.borrow_mut();
        state.inc();
       
        }
    });
    let mut dec_button = Button::default().with_label("-");
    dec_button.set_callback( {
        let state = state.clone();        
        move |_| {
        let mut state = state.borrow_mut();
        state.dec();
        }
    });


    window.resizable(&col);
    col.end();
    window.end();
    window.show();

    app.run().unwrap();
}
