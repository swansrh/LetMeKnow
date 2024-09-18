use chrono;
use chrono::FixedOffset;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::stdout;
//use std::io::Result;
use std::io::Write;
use std::path::Path;

//ratatui imports
use color_eyre::Result;
use itertools::Itertools;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin, Rect},
    style::{self, Color, Modifier, Style, Stylize},
    text::{Line, Text},
    widgets::{
        Block, BorderType, Cell, HighlightSpacing, Paragraph, Row, Scrollbar, ScrollbarOrientation,
        ScrollbarState, Table, TableState,
    },
    DefaultTerminal, Frame,
};
use style::palette::tailwind;
use unicode_width::UnicodeWidthStr;

const PALETTES: [tailwind::Palette; 1] = [tailwind::BLUE];
const INFO_TEXT: &str = "(Esc) quit | (↑) move up | (↓) move down";
const ITEM_HEIGHT: usize = 4;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Task {
    //struct for the task details will be used for reading and writing to json file
    task_id: String,
    task_name: String,
    task_details: String,
    stake_holder: String,
    due_date: String,
    date_created: String,
    state: String,
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
        [
            &self.task_id,
            &self.task_name,
            &self.task_details,
            &self.stake_holder,
            &self.due_date,
            &self.date_created,
            &self.state,
        ]
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

//RATATUI *****************************************************************************************************************************************************************************
use std::error::Error;

fn main() -> Result<()> {
    //blanking out actual functions to begin testing new TUI
    create_backup();
    logo_print(); //prints the logo at the begining of the script
    show_tasks("./data.json".to_string());
    main_menu();
    delete_file("./backup.json".to_string());




    //the retun here is required by ratatui
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}

fn main_menu() {
    loop {
        //begining of loop for menu
        println!("\nWhat would you like to do? (Type 'Help' for help)");
        let user_input = get_input(); //gets user input
                                      //println!("{}", user_input); //just for debug to see what was input by the user

        match user_input.as_str() {
            "Help" => help_menu(),
            "help" => help_menu(),
            "Show" => show_tasks("./data.json".to_string()),
            "show" => show_tasks("./data.json".to_string()),
            "Edit" => println!("Edit a task"),
            "edit" => println!("Edit a task"),
            "Detail" => show_details(),
            "Details" => show_details(),
            "details" => show_details(),
            "detail" => show_details(),
            "remove" => remove_task(),
            "Remove" => remove_task(),
            "delete" => remove_task(),
            "Delete" => remove_task(),
            "add" => add_task(),
            "Add" => add_task(),
            "archive" => show_tasks("./archive.json".to_string()),
            "Archive" => show_tasks("./archive.json".to_string()),
            "Exit" => break,
            "exit" => break,
            "q" => break,
            "Q" => break,
            "" => println!("Silly goose, you have to input something"),
            _ => println!(
                "Please input a valid option. Type 'Help' for a list of available commands."
            ),
        }
    } // loop end
}

fn add_task() {
    //******************          ADD FUNCTIONS        ****************************************************************************************************************
    // create a new task and append it to the JSON file "./data.json"
    let mut tasks: Vec<Task> = read_json("./data.json".to_string());
    let mut temp_inputs = Task {
        task_id: read_task_id(),
        date_created: get_date_time(),
        state: "Created".to_string(),
        ..Default::default()
    }; //default values set

    for n in 1..5 {
        match n {
            1 => temp_inputs.task_name = add_input("task name".to_string()), //make the variable we want = a function
            2 => temp_inputs.task_details = add_input("task details".to_string()),
            3 => temp_inputs.due_date = add_input("due date".to_string()),
            4 => temp_inputs.stake_holder = add_input("the main stake holder".to_string()),
            _ => println!("This is for use case nothing"),
        }
    }

    //print the task and ask if the user wants to go forth
    println!("\nPlease see your new task details below:");
    println!("\nTask ID: {} \nTask: {}\nDetails: {}\nStake Holder: {}\nDate Created: {}\nDate Due: {}\nCurrent State:  {}", temp_inputs.task_id, temp_inputs.task_name, temp_inputs.task_details, temp_inputs.stake_holder, temp_inputs.date_created, temp_inputs.due_date, temp_inputs.state);
    println!("\nWould you like to save this task?");
    let confirm = get_input();

    let mut go_forth = false;
    match confirm.as_str() {
        "y" => go_forth = true,
        "yes" => go_forth = true,
        "okay" => go_forth = true,
        "ok" => go_forth = true,
        "Yes" => go_forth = true,
        "Y" => go_forth = true,
        "n" => println!("cancelling and returning to main menu"),
        "N" => println!("cancelling and returning to main menu"),
        "No" => println!("cancelling and returning to main menu"),
        "no" => println!("cancelling and returning to main menu"),
        "Nope" => println!("cancelling and returning to main menu"),
        "nope" => println!("cancelling and returning to main menu"),
        "NO" => println!("cancelling and returning to main menu"),
        "NOPE" => println!("cancelling and returning to main menu"),
        _ => println!("Please input something valid"),
    }

    if go_forth == true {
        tasks.push(temp_inputs);
        let json_converted = serde_json::to_string(&tasks).expect("Could not convert data to JSON");
        overwrite_existing(json_converted, "./data.json"); //this function saves to file
        overwrite_existing(read_task_id(), "./count.txt");
        println!("Task was saved")
    }
}

fn get_date_time() -> String {
    //gets the current date time of UTC +10
    let offset = FixedOffset::east_opt(10 * 60 * 60).unwrap(); //date time offset for chrono
    let dt = chrono::Utc::now().with_timezone(&offset); //gets current date time in UTC +10 (Australia). Change the first # in the secs to the utc offset
    dt.to_string()
}

fn read_task_id() -> String {
    //returns the next task id as a string
    let count_content = fs::read_to_string("./count.txt").expect("Unable to read the file"); //reads the existing counter and parses it to an INT for manuipulation
    let mut new_id: i32 = count_content
        .parse()
        .expect("Could not convert nummber to an integer"); //this will be changed later to the new latest id and saved back to the count.txt file
    new_id += 1;
    new_id.to_string()
}

fn add_input(input_for: String) -> String {
    //gets input from user and returns the value to be saved against the temp struct in the add_task() function
    println!("Please input the {}", input_for);
    let user_input = get_input();
    //println!("User Input is '{}'", user_input); //debugging
    user_input
}

fn get_input() -> String {
    loop {
        let mut s: String = String::new();
        io::stdin().read_line(&mut s).expect("Failed to read line");

        if s.ends_with("\n") {
            //these statements remove the newline characters at the end of the string
            s.pop();
            if s.ends_with("\r") {
                s.pop();
            }
        }

        if s.is_empty() {
            //checking if the input is empty or not. No error checking on type
            println!("Input cannot be empty");
        } else {
            return s;
        }
    }
}

fn show_tasks(file_path: String) {
    //******************          SHOW FUNCTIONS        ****************************************************************************************************************
    //used to show the tasks
    println!("TASK ID, TASK NAME, STAKE HOLDER, DUE DATE, STATE");
    let tasks: Vec<Task> = read_json(file_path);

    for lines in tasks {
        println!(
            "{} | {} | {} | {}",
            lines.task_id, lines.task_name, lines.state, lines.due_date
        );
    }
}

fn read_json(file_path: String) -> Vec<Task> {
    //add an input for file path so that the archive file can also be read.

    let json_file_path = Path::new(&file_path); //file path of json
    let data_file: File = File::open(json_file_path).expect("File not found"); //opens the json file
                                                                               //println!("HERE");//debugging
    let tasks: Vec<Task> =
        serde_json::from_reader(data_file).expect("Error while reading ./data.json");
    tasks
}

fn show_details() {
    //shows the details of the
    println!("Which task would you like to see in detail? (Press Q to go back)");
    let user_input = get_input(); //user input

    match user_input.as_str() {
        //checks user input to deterimine next action
        "q" => println!("Cancelled"),
        "Q" => println!("Cancelled"),
        _ => check_matching_task(user_input), //Takes string and feeds it to the matching function to return a task and its details
    }
}

fn check_matching_task(input: String) {
    let data: Vec<Task> = read_json("./data.json".to_string());
    let mut is_matching = false;

    for lines in data {
        if lines.task_id.as_str() == input.as_str() {
            println!("\nTask ID: {} \nTask: {}\nDetails: {}\nStake Holder: {}\nDate Created: {}\nDate Due: {}\nCurrent State:  {}", lines.task_id, lines.task_name, lines.task_details, lines.stake_holder, lines.date_created, lines.due_date, lines.stake_holder);
            is_matching = true;
        }
    }

    if is_matching == false {
        println!("\nNo Matching task ID was found.")
    }
}

fn remove_task() {
    //******************          REMOVE FUNCTIONS        ****************************************************************************************************************
    //This is for removing tasks from the list and saving that list over the original list
    println!("Please input the task ID you would like to remove (Press Q to go back)");
    let user_input = get_input(); //user input

    match user_input.as_str() {
        //checks user input to deterimine next action
        "q" => println!("Cancelled"),
        "Q" => println!("Cancelled"),
        _ => check_for_removal(&user_input), //Takes string and feeds it to the matching function to return a task and its details
    }
}

fn return_task_index(input: &String) -> usize {
    //needs to return int 32 that is the index of the task in the vector
    let data_test: Vec<Task> = read_json("./data.json".to_string()); //reads the json file and returns the struct HERE FOR TESTING
    let mut index_of_task = 0;

    for lines in data_test {
        if lines.task_id.as_str() == input.as_str() {
            return index_of_task;
        }
        index_of_task += 1;
    }
    index_of_task
}

fn check_if_task_exists(input: &String) -> bool {
    let data: Vec<Task> = read_json("./data.json".to_string());
    let mut is_matching = false;

    for lines in data {
        if lines.task_id.as_str() == input.as_str() {
            is_matching = true;
        }
    }

    if is_matching == false {
        println!("\nNo Matching task ID was found.")
    }
    is_matching
}

fn overwrite_existing(text_to_write: String, file_path: &str) {
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    let _ = f.write_all(&text_to_write.as_bytes());
}

fn help_menu() {
    //******************          HELP MENU        ****************************************************************************************************************
    logo_print();
    println!("\nLetMeKnow Version 0.0.1 Windows Build\nThese commands have been defined internally. Type 'Help' at anytime to see this list.");
    println!("\nShow\n    Shows the user currently active tasks");
    println!("\nDetails\n     Navigates to a specific task to get additional information");
    println!("\nAdd\n     Allows the user to add an aditional task to the list");
    println!("\nEdit\n     Allows a user to edit an existing task");
    println!("\nRemove\n     Remopoves an existing task from the list");
    println!("\nExit\n     Quits the program");
}

fn create_backup() {
    //******************          BACKUP FUNCTIONS        ****************************************************************************************************************
    let tasks: Vec<Task> = read_json("./data.json".to_string());
    let json_converted = serde_json::to_string(&tasks).expect("Could not convert data to JSON");
    create_new(json_converted);
}

fn create_new(write_to_file: String) {
    //creates a new file and saves json to it
    let mut file =
        File::create("./backup.json").expect("Could not create backup file. Terminating");
    file.write_all(write_to_file.as_bytes())
        .expect("Could not write to backup file. Terminating. Please delete existing backup");
}

fn delete_file(file_path: String) {
    fs::remove_file(file_path).expect("Could not remove file.")
}

fn logo_print() {
    //******************          LOGO        ****************************************************************************************************************************************
    //Prints the logo
    println!("          _                      _            ");
    println!("         (_)                    | |           ");
    println!("__      ___ _ __ ___  _ __    __| | _____   __");
    println!("\\ \\ /\\ / / | '__/ _ \\| '_ \\  / _` |/ _ \\ \\ / /");
    println!(" \\ V  V /| | | | (_) | | | || (_| |  __/\\ V / ");
    println!("  \\_/\\_/ |_|_|  \\___/|_| |_(_)__,_|\\___| \\_/  ");
    println!("\nWelcome to Let Me know, the task management CLI tool")
}

fn check_for_removal(input: &String) {
    let mut archived_tasks: Vec<Task> = read_json("archive.json".to_string()); //reads the archive.json file
    let mut data: Vec<Task> = read_json("./data.json".to_string()); //reads the json file and returns the struct
    let task_exists: bool = check_if_task_exists(&input);
    let file_path: &str = "./data.json";

    if task_exists == true {
        println!("Task exists and can be removed"); //if it exists, find it and return the index. then remove it
        let task_index = return_task_index(&input);

        let temp_task: Task = data.swap_remove(task_index);
        archived_tasks.push(temp_task);
        let json_converted =
            serde_json::to_string(&archived_tasks).expect("Could not convert data to JSON");
        overwrite_existing(json_converted, "archive.json");

        let json_converted = serde_json::to_string(&data).expect("Could not convert data to JSON");
        overwrite_existing(json_converted, file_path); //this function saves to file
    } else {
        println!("Task does not exist"); //Let's the user know the task does not exist and returns them to the main menu
    }
}

//ratatui below
struct TableColors {
    buffer_bg: Color,
    header_bg: Color,
    header_fg: Color,
    row_fg: Color,
    selected_style_fg: Color,
    normal_row_color: Color,
    alt_row_color: Color,
    footer_border_color: Color,
}

impl TableColors {
    const fn new(color: &tailwind::Palette) -> Self {
        Self {
            buffer_bg: tailwind::SLATE.c950,
            header_bg: color.c900,
            header_fg: tailwind::SLATE.c200,
            row_fg: tailwind::SLATE.c200,
            selected_style_fg: color.c400,
            normal_row_color: tailwind::SLATE.c950,
            alt_row_color: tailwind::SLATE.c900,
            footer_border_color: color.c400,
        }
    }
}

struct App {
    state: TableState,
    items: Vec<Task>,
    longest_item_lens: (u16, u16, u16, u16, u16), // order is (task_id, task_name, task_details, due_date, state)
    scroll_state: ScrollbarState,
    colors: TableColors,
    color_index: usize,
}

impl App {
    fn new() -> Self {
        let data_vec = read_json("./data.json".to_string()); //Feed it the info it needs
        Self {
            state: TableState::default().with_selected(0),
            longest_item_lens: constraint_len_calculator(&data_vec),
            scroll_state: ScrollbarState::new((data_vec.len() - 1) * ITEM_HEIGHT),
            colors: TableColors::new(&PALETTES[0]),
            color_index: 0,
            items: data_vec,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn set_colors(&mut self) {
        self.colors = TableColors::new(&PALETTES[self.color_index]);
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('j') | KeyCode::Down => self.next(),
                        KeyCode::Char('k') | KeyCode::Up => self.previous(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(3)]);
        let rects = vertical.split(frame.area());

        self.set_colors();

        self.render_table(frame, rects[0]);
        self.render_scrollbar(frame, rects[0]);
        self.render_footer(frame, rects[1]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default()
            .fg(self.colors.header_fg)
            .bg(self.colors.header_bg);
        let selected_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .fg(self.colors.selected_style_fg);

        let header = [
            "ID",
            "Task Name",
            "Task Details",
            "Stakeholder",
            "Due Date",
            "Date Created",
            "State",
        ] //this defines the first row which is the header
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(header_style)
        .height(1);

        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let color = match i % 2 {
                0 => self.colors.normal_row_color,
                _ => self.colors.alt_row_color,
            };
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(format!("\n{content}\n"))))
                .collect::<Row>()
                .style(Style::new().fg(self.colors.row_fg).bg(color))
                .height(4)
        });
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                // + 1 is for padding.                              //NEED TO ADD CONSTRAINT FOR EACH COLUMN I WANT TO DISPLAY HERE
                Constraint::Length(self.longest_item_lens.0 + 1),
                Constraint::Min(self.longest_item_lens.1 + 1),
                Constraint::Min(self.longest_item_lens.2),
                Constraint::Min(self.longest_item_lens.1 + 1),
                Constraint::Min(self.longest_item_lens.1 + 1),
                Constraint::Min(self.longest_item_lens.1 + 1),
                Constraint::Min(self.longest_item_lens.0),
            ],
        )
        .header(header)
        .highlight_style(selected_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .bg(self.colors.buffer_bg)
        .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.state);
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let info_footer = Paragraph::new(Line::from(INFO_TEXT))
            .style(
                Style::new()
                    .fg(self.colors.row_fg)
                    .bg(self.colors.buffer_bg),
            )
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
                    .border_style(Style::new().fg(self.colors.footer_border_color)),
            );
        frame.render_widget(info_footer, area);
    }
}

fn constraint_len_calculator(items: &[Task]) -> (u16, u16, u16, u16, u16) {
    let id_len = items //this implements the task ID
        .iter()
        .map(Task::task_id)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let name_len = items //this implements the task name
        .iter()
        .map(Task::task_name)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let details_len = items //this implements the task details
        .iter()
        .map(Task::task_details)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let due_date = items //this implements the task name
        .iter()
        .map(Task::due_date)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    let state_len = items //this implements the task name
        .iter()
        .map(Task::state)
        .flat_map(str::lines)
        .map(UnicodeWidthStr::width)
        .max()
        .unwrap_or(0);

    #[allow(clippy::cast_possible_truncation)]
    (
        id_len as u16,
        name_len as u16,
        details_len as u16,
        due_date as u16,
        state_len as u16,
    )
}