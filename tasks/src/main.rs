use chrono;
use chrono::FixedOffset;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::stdout;
//use std::io::Result;
use std::io::Write;
use std::path::Path;

use std::{error::Error, io};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, ITEM_HEIGHT},
    ui::{ui, Task},
};

//RATATUI *****************************************************************************************************************************************************************************

fn main() -> Result<(), Box<dyn Error>>{
    //blanking out actual functions to begin testing new TUI
    //create_backup();
    //logo_print(); //prints the logo at the begining of the script
    //show_tasks("./data.json".to_string());
    //main_menu();
    //delete_file("./backup.json".to_string());
    
    
    enable_raw_mode()?;
    let mut stderr = io::stderr(); //stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    //create app and run
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

     // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res { //checks if run_app() errored or returned OK
        if do_print {
            //app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {

    loop { //draws the UI to the terminal
        terminal.draw(|f| ui(f, app))?;

       if let Event::Key(key) = event::read()? {
        //if key.kind == event::KeyEventKind::Release { //this if statement skips any event that is not a key press
        //    continue;
        //}
            if key.kind == KeyEventKind::Press {
                    match app.current_screen{
                        CurrentScreen::table_screen => match key.code {
                            KeyCode::Delete => {
                                return Ok(true);
                            }
                            KeyCode::Down => {
                                next(app)
                            }
                            KeyCode::Up => {
                                previous(app);
                            }
                            KeyCode::Enter => {
                                app.current_screen = CurrentScreen::detail_screen;
                            }
                            KeyCode::Char('n') => {
                                app.current_screen = CurrentScreen::new_screen;
                            }

                            _ => {}
                        }
                        CurrentScreen::detail_screen => match key.code {
                            KeyCode::Delete => {
                                return Ok(true);
                            }
                            KeyCode::Esc => {
                                app.current_screen = CurrentScreen::table_screen;
                            }
                            _ => {}
                        },
                        CurrentScreen::splash_screen => match key.code {
                            KeyCode::Delete => {
                                return Ok(true);
                            }
                            KeyCode::Enter => {
                                app.current_screen = CurrentScreen::table_screen;
                            }
                            
                            _ => {}
                        }
                        CurrentScreen::new_screen => match key.code {
                            KeyCode::Esc => {
                                app.current_screen = CurrentScreen::table_screen;
                            }
                            KeyCode::Enter => {
                                app.current_screen = CurrentScreen::table_screen;
                            }
                            KeyCode::Delete => {
                                return Ok(true);
                            }
                            _ => {}
                        }
                    }
                    
            }
        }
    }        
}

pub fn next(app: &mut App) {
    let i = match app.table_state.selected() {
        Some(i) => {
            if i >= app.items.len() - 1 {
                0
            }else {
                i + 1
            }
        }
        None => 0,
    };
    app.table_state.select(Some(i));
    app.scroll_state = app.scroll_state.position(i * ITEM_HEIGHT);

}

pub fn previous(app: &mut App) {
    let i = match app.table_state.selected() {
        Some(i) => {
            if i == 0 {
                app.items.len() -1
            }else {
                i - 1
            }
        }
        None => 0,
    };
    app.table_state.select(Some(i));
    app.scroll_state = app.scroll_state.position(i * ITEM_HEIGHT);
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

pub fn read_json(file_path: String) -> Vec<Task> {
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

