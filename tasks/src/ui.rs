use color_eyre::Result;
use color_eyre::owo_colors::OwoColorize;
use itertools::Itertools;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin, Rect, Direction},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text, Span},
    widgets::{
        Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, TableState,
    },
    DefaultTerminal, Frame,
};

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use crate::app::{App, CurrentScreen};

#[derive(Debug, Deserialize, Serialize, Clone)] //data type of the data being imported
pub struct Task {//all of these must be public to be used with the other functions found in main.rs
    pub task_id: String,
    pub task_name: String,
    pub task_details: String,
    pub stake_holder: String,
    pub due_date: String,
    pub date_created: String,
    pub state: String,
}

impl Default for Task {
    //This implements default values for the Task type. This is a default set of values that can be used to intialiuse the variable. Used for the creation Process https://gist.github.com/ChrisWellsWood/84421854794037e760808d5d97d21421
    fn default() -> Task {
        Task {
            task_id: "Task ID".to_string(),
            task_name: "Task Name".to_string(),
            task_details: "Additional Details".to_string(),
            stake_holder: "Stake Holders".to_string(),
            due_date: "Date Due".to_string(),
            date_created: "Date Due".to_string(),
            state: "Current State".to_string(),
        }
    }
}


pub fn ui(frame: &mut Frame, app: &App) {
    //let vertical = Layout::vertical([Constraint::Min(5), Constraint::Max(3)]);
    let rects = frame.area();

    let rows = read_json("./data.json".to_string());
    let other_rows = [
        Row::new(vec!["Cell1", "Cell2", "Cell3"]),
        Row::new(vec!["Cell4", "Cell5", "Cell6"]),
        Row::new(vec!["Cell7", "Cell8", "Cell9"]),
        Row::new(vec!["Cell10", "Cell11", "Cell12"]),
        Row::new(vec!["Cell3", "Cell14", "Cell15"]),
        Row::new(vec!["Cell16", "Cell17", "Cell18"]),
        ];
 
    let width = [
        Constraint::Length(5),
        Constraint::Length(5),
        Constraint::Length(5),
    ];

    let table = Table::new(other_rows, width)
        .column_spacing(1)
        .style(Style::new().blue())
        .header(
            //Row::new(vec!["ID", "Name", "Details", "StakeHolder", "Due", "Created", "State"])
            Row::new(vec!["ID", "Name", "Details"])
            .style(Style::new().bold())
            .bottom_margin(1),
            )
            .footer(Row::new(vec!["(Del) Quit"]))
            .block(Block::new().title("LetMeKnow"))
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>");
    
    if let CurrentScreen::table_screen = app.current_screen{ //ui components for the table  
        let area = rects;
        frame.render_widget(table, area);
    }

}

pub fn read_json(file_path: String) -> Vec<Task> {
    //add an input for file path so that the archive file can also be read.

    let json_file_path = Path::new(&file_path); //file path of json
    let data_file: File = File::open(json_file_path).expect("File not found"); //opens the json file
                                                                               //println!("HERE");//debugging
    let tasks: Vec<Task> =
        serde_json::from_reader(data_file).expect("Error while reading ./data.json");
    tasks
}