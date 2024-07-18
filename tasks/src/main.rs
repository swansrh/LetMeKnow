use std::io;
use std::path::Path;
use std::fs::File;
use serde::Deserialize;
use std::env;

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
        "Edit" => println!("Edit a task"),
        "edit" => println!("Edit a task"),
        "Detail" => println!("Detail view"),
        "detail" => println!("Detail view"),
        "Exit" => break,
        "exit" => break,
        "q" => break,
        "Q" => break,
        _ => println!("Please input a valid option. Type 'Help' for a list of available commands."),
    }
    }// while loop end
}

fn get_input() -> String{
    let mut s = String::new(); 
    io::stdin().read_line(&mut s).expect("Failed to read line");
    
    if s.ends_with("\n") { //these statements remove the newline characters at the end of the string
        s.pop();
        //println!("Removed1"); //used for debug
        if s.ends_with("\r") {
            s.pop();
            //println!("Removed2"); //used for debug
        }
    }
    return s;
}

fn show_tasks() {//used to show the tasks
    //let path = env::current_dir().expect("FAILED TO GET"); //gets CWD 
    //println!("The current directory is {}", path.display()); //Prints current CWD to console
    
    let json_file_path = Path::new("fakeData.json"); //file path of json
    
    let data_file = File::open(json_file_path).expect("File not found"); //opens the json file
    println!("HERE");//debugging
    let tasks:Vec<Task> = serde_json::from_reader(data_file).expect("Error while reading data.json");
    
    println!("TASK ID, TASK NAME, STAKE HOLDER, DUE DATE, STATE");
    for lines in tasks {
        println!("YES");
    }
}

fn help_menu() {
    logo_print();
    println!("\nLetMeKnow Version 0.0.1 Windows Build\nThese commands have been defined internally. Type 'Help' at anytime to see this list.");
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