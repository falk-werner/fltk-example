use fltk::app::Screen;
use fltk::{group, menu};
use fltk::{app, prelude::*, window::Window};
use fltk::enums::Shortcut;

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;

const MENU_HEIGHT : i32 = 30;
const MENU_FILE_NEW: &str = "&File/&New\t";
const MENU_FILE_QUIT: &str = "&File/&Quit\t";
const MENU_HELP_ABOUT: &str = "&Help/&About\t";

fn main() {
    let screen = Screen::new(0).expect("Failed to finde primary screen");

    let app = app::App::default();
    let mut wind = Window::default()
        .with_size(WIDTH, HEIGHT)
        .with_pos((screen.w() - WIDTH) / 2, (screen.h() - HEIGHT) / 2)
        .with_label("FLTK Menu Example");

    let mut col = group::Flex::default_fill().column();
    col.set_pad(0);
    let mut main_menu = menu::SysMenuBar::default();
    main_menu.add(MENU_FILE_NEW, 
        Shortcut::Ctrl | 'n', 
        menu::MenuFlag::MenuDivider, 
        menu_callback);
    main_menu.add(MENU_FILE_QUIT, 
        Shortcut::Ctrl | 'q', 
        menu::MenuFlag::Normal, 
        menu_callback);
        main_menu.add(MENU_HELP_ABOUT, 
            Shortcut::None, 
            menu::MenuFlag::Normal, 
            menu_callback);
        main_menu.end();
    wind.resizable(&col);
    col.fixed(&main_menu, MENU_HEIGHT);
    col.end();

    wind.end();
    wind.show();


    app.run().unwrap();
}

fn menu_callback(main_menu: &mut impl MenuExt) {
    if let Ok(menu_path) = main_menu.item_pathname(None) {
        match menu_path.as_str() {
            MENU_FILE_NEW => {
                println!("file -> new");
            }
            MENU_FILE_QUIT => {
                println!("file -> quit");
                app::quit();
            }
            MENU_HELP_ABOUT => {
                println!("help -> about");
            }
            _ => {}
        }
    }
}