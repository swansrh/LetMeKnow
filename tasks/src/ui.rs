use color_eyre::Result;
use color_eyre::owo_colors::OwoColorize;
use itertools::Itertools;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin, Rect, Direction},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Text, Span},
    widgets::{
        Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, TableState, Borders,
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


pub fn ui(frame: &mut Frame, app: &App) {//defines the split in the layout
    let rects = frame.area(); 
    let footer_text = "(Del) Exit Program / (N) New Entry / (Ent) Details Page";

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
            Row::new(vec!["ID", "Name", "Details", "Stake", "Due", "Created", "state"])
                .style(Style::new().bold())
                .bottom_margin(0),
            )
            .highlight_style(Style::new().reversed())
            .highlight_symbol(">>");
    
    let footer = Paragraph::new(Line::from(footer_text))
        .style(
            Style::new()
                .bg(Color::Green)
                .fg(Color::White),
            )
        .centered()
        .block(
            Block::bordered()
                .border_type(BorderType::Double)
                .border_style(Style::new().fg(Color::Blue))
        );

    if let CurrentScreen::table_screen = app.current_screen{ //This checks whether the table is the current screen
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(3)]);
        let recters = vertical.split(frame.area());
        
        let area_top = recters[0];
        let area_bottom = recters[1];
        frame.render_widget(table, area_top);
        frame.render_widget(footer, area_bottom);    
    }

    if let CurrentScreen::splash_screen = app.current_screen{ //this checks whether the splash screen is currently active
        //splash screen UI here
        let splash_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(50),
            Constraint::Percentage(50)
        ])
        .split(frame.area()); 

        let b = Block::default() //block for top half
            .borders(Borders::ALL)
            .title("Top Half");
        
        let other_b = Block::default() //block for bottom half
            .borders(Borders::ALL)
            .title("Bottom Half");

        let area = splash_layout[0];
        let area_two = splash_layout[1];
        
        frame.render_widget(b, area);
        frame.render_widget(other_b, area_two);
    }
}

pub fn read_json(file_path: String) -> Vec<Task> {
    let json_file_path = Path::new(&file_path); //file path of json
    let data_file: File = File::open(json_file_path).expect("File not found"); //opens the json file
                                                                               //println!("HERE");//debugging
    let tasks: Vec<Task> =
        serde_json::from_reader(data_file).expect("Error while reading ./data.json");
    tasks
}