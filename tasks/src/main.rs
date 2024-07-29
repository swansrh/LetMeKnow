use std::io;
use std::path::Path;
use std::fs::File;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Task { //struct for the task details will be used for reading and writing to json file
    task_id: String,
    task_name: String,
    task_details: String,
    stake_holder: String,
    due_date: String,
    date_created: String,
    state: String
}

fn main() {
    logo_print(); //prints the logo at the begining of the script
    show_tasks();
    main_menu();
}

fn main_menu() {

    loop{//begining of loop for menu
    println!("\nWhat would you like to do? (Type 'Help' for help)");
    let user_input = get_input(); //gets user input
    //println!("{}", user_input); //just for debug to see what was input by the user

    match user_input.as_str(){
        "Help" => help_menu(),
        "help" => help_menu(),
        "Show" => show_tasks(),
        "show" => show_tasks(),
        "Edit" => println!("Edit a task"),
        "edit" => println!("Edit a task"),
        "Detail" => show_details(),
        "detail" => show_details(),
        "Exit" => break,
        "exit" => break,
        "q" => break,
        "Q" => break,
        _ => println!("Please input a valid option. Type 'Help' for a list of available commands."),
    }
    }// loop end
}

fn get_input() -> String{
    let mut s = String::new(); 
    io::stdin().read_line(&mut s).expect("Failed to read line");
    
    if s.ends_with("\n") { //these statements remove the newline characters at the end of the string
        s.pop();
        if s.ends_with("\r") {
            s.pop();
        }
    }
    return s;
}

fn show_tasks() {//used to show the tasks
    println!("TASK ID, TASK NAME, STAKE HOLDER, DUE DATE, STATE");
    let tasks:Vec<Task> = read_json();
    
    for lines in tasks {
        println!("{} | {} | {} | {}", lines.task_id, lines.task_name, lines.state, lines.due_date);
    }
}

fn read_json() -> Vec<Task> {
    let json_file_path = Path::new("fakeData.json"); //file path of json
    
    let data_file = File::open(json_file_path).expect("File not found"); //opens the json file
    //println!("HERE");//debugging
    let tasks:Vec<Task> = serde_json::from_reader(data_file).expect("Error while reading data.json");
    tasks
}

fn show_details() {//shows the details of the 
    println!("Which task would you like to see in detail? (Press Q to go back)");
    let user_input = get_input();//user input
    
    match user_input.as_str(){ //checks user input to deterimine next action
        "q" => println!("Cancelled"),
        "Q" => println!("Cancelled"),
        _ => check_matching_task(user_input), //need to feed this is a string so that the matching task can do pattern matching
    }
}

fn check_matching_task(input: String){
    let data:Vec<Task> = read_json();
    let mut is_matching = false;
    
    for lines in data{
        if lines.task_id.as_str() == input.as_str(){
            println!("\nTask ID: {} \nTask: {}\nDetails: {}\nStake Holder: {}\nDate Created: {}\nDate Due: {}\nCurrent State:  {}", lines.task_id, lines.task_name, lines.task_details, lines.stake_holder, lines.date_created, lines.due_date, lines.stake_holder);
            is_matching = true;
        }
    }

    if is_matching == false {
        println!("\nNo Matching task ID was found.")
    }
}

fn help_menu() {
    logo_print();
    println!("\nLetMeKnow Version 0.0.1 Windows Build\nThese commands have been defined internally. Type 'Help' at anytime to see this list.");
    println!("\nShow\n    Shows the user currently active tasks");
    println!("\nAdd\n     Allows the user to add an aditional task to the list");
    println!("\nEdit\n     Allows a user to edit an existing task");
    println!("\nExit\n     Quits the program");
    println!("\nDetails\n     Navigates to a specific task to get additional information");
}

fn logo_print() { //Prints the logo
    println!("          _                      _            ");
    println!("         (_)                    | |           ");
    println!("__      ___ _ __ ___  _ __    __| | _____   __");
    println!("\\ \\ /\\ / / | '__/ _ \\| '_ \\  / _` |/ _ \\ \\ / /");
    println!(" \\ V  V /| | | | (_) | | | || (_| |  __/\\ V / ");
    println!("  \\_/\\_/ |_|_|  \\___/|_| |_(_)__,_|\\___| \\_/  ");
    println!("\nWelcome to Let Me know, the task management CLI tool")
}