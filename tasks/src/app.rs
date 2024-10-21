use ratatui::widgets::{ScrollbarState, TableState};
//use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;

use crate::{
    ui::{ui, Task},
};

pub const ITEM_HEIGHT: usize = 4;

pub enum CurrentlyEditing {
    First,
    Second,
    Third,
}

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
    pub first_input: String, //inputOne
    pub second_input: String, //inputTwo
    pub third_input: String, //inputThree
    pub currently_editing: Option<CurrentlyEditing> //keeps tab what is currently editing
}

impl App{
    pub fn new() -> App {
        let data_raw = read_json("./data.json".to_string()); //reads in the json file
        App {
            current_screen: CurrentScreen::splash_screen,
            table_state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::new((data_raw.len() - 1) * ITEM_HEIGHT),
            items: data_raw, //this must go last otherwise you get burrowing issues as the ownership changes
            first_input: String::new(),
            second_input: String::new(),
            third_input: String::new(),
            currently_editing: None,
        }
    }

    pub fn save_values(&mut self) {
        //right here is where the write to file function needs to go
        //take the inputs as strings in the input. Here, serialise them as the task struct and save them to the json file.        
        self.first_input = String::new();
        self.second_input = String::new();
        self.third_input = String::new();

        self.currently_editing = None; //clears the values so the same ones are not there
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::First => self.currently_editing = Some(CurrentlyEditing::Second),
                CurrentlyEditing::Second => self.currently_editing = Some(CurrentlyEditing::Third),
                CurrentlyEditing::Third => self.currently_editing = Some(CurrentlyEditing::First),
            } 
        } else {
            self.currently_editing = Some(CurrentlyEditing::First);
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