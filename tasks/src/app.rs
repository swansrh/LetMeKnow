use ratatui::widgets::{ScrollbarState, TableState};
//use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

use crate::{
    ui::{ui, Task},
};

pub const ITEM_HEIGHT: usize = 4;

pub enum CurrentScreen {
    table_screen,
    detail_screen,
    splash_screen,
    new_screen,
}

struct table_screen{

}

struct new_screen{
    
}

struct detail_screen{

}

struct splash_screen{

}

pub struct App{
    pub current_screen: CurrentScreen,
    pub table_state: TableState,
    pub items: Vec<Task>,
    pub scroll_state: ScrollbarState,
}

impl App{
    pub fn new() -> App {
        let data_raw = read_json("./data.json".to_string()); //reads in the json file
        App {
            current_screen: CurrentScreen::splash_screen,
            table_state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new((data_raw.len() - 1) * ITEM_HEIGHT),
            items: data_raw, //this must go last otherwise you get burrowing issues as the ownership changes
        }
    }
}

fn read_json(file_path: String) -> Vec<Task> {
    let json_file_path = Path::new(&file_path); //file path of json
    let data_file: File = File::open(json_file_path).expect("File not found"); //opens the json file
                                                                               //println!("HERE");//debugging
    let tasks: Vec<Task> =
        serde_json::from_reader(data_file).expect("Error while reading ./data.json");
    tasks
}