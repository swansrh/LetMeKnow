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

impl Task {
    const fn ref_array(&self) -> [&String; 7] {
        [&self.task_id, &self.task_name, &self.task_details, &self.stake_holder, &self.due_date, &self.date_created, &self.state]
    }

    fn task_id(&self) -> &str {
        &self.task_id
    }

    fn task_name(&self) -> &str {
        &self.task_name
    }

    fn task_details(&self) -> &str {
        &self.task_details
    }

    fn stake_holder(&self) -> &str {
        &self.stake_holder
    }

    fn due_date(&self) -> &str {
        &self.due_date
    }

    fn date_created(&self) -> &str {
        &self.date_created
    }

    fn state(&self) -> &str {
        &self.state
    }
}


pub fn ui(frame: &mut Frame, app: &App) {
    //let vertical = Layout::vertical([Constraint::Min(5), Constraint::Max(3)]);
    let rects = frame.area();

    let data_raw = read_json("./data.json".to_string()); //reads in the json file
    let more_rows = data_raw.iter().enumerate().map(|(i, data)| { //enumrates over each line of the json
        let item = data.ref_array(); //removes ID fields and just has the data
        item.into_iter() //loop
            .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))//this is where it breaks it down to cells and creates the rows
            .collect::<Row>()
            .height(2)
    });
    let width = [
        Constraint::Length(5 + 1),
        Constraint::Length(25 + 1),
        Constraint::Length(130 + 1),
        Constraint::Length(20 + 1),
        Constraint::Length(10 + 1),
        Constraint::Length(10 + 1),
        Constraint::Length(10),
    ];

    let table = Table::new(more_rows, width)
        .column_spacing(1)
        .style(Style::new().blue())
        .header(
            //Row::new(vec!["ID", "Name", "Details", "StakeHolder", "Due", "Created", "State"])
            Row::new(vec!["ID", "Name", "Details", "Stake", "Due", "Created", "state"])
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