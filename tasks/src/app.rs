pub enum CurrentScreen {
    table_screen,
    detail_screen,
    splash_screen,
}

struct table_screen{

}

struct detail_screen{

}

struct splash_screen{

}

pub struct App{
    pub current_screen: CurrentScreen,
}

impl App{
    pub fn new() -> App {
        App {
            current_screen: CurrentScreen::table_screen,
        }
    }
}