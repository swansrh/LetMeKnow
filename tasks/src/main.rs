use serde::Deserialize;
use serde::Serialize;
//use std::default;
//use serde_json::from_str;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
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

impl Default for Task { //This implements default values for the Task type. This is a default set of values that can be used to intialiuse the variable. Used for the creation Process https://gist.github.com/ChrisWellsWood/84421854794037e760808d5d97d21421
    fn default() -> Task {
        Task{task_id: "Task ID".to_string(), task_name: "Task Name".to_string(), task_details: "Additional Details".to_string(), stake_holder: "Stake Holders".to_string(),due_date: "Date Due".to_string(), date_created: "Todays Date".to_string(),state: "Current State".to_string()}
    }
}
fn main() {
    logo_print(); //prints the logo at the begining of the script
    show_tasks();
    main_menu();
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
            "Show" => show_tasks(),
            "show" => show_tasks(),
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

fn add_task() {// create a new task and append it to the JSON file "fakeData.json"
    let mut data: Vec<Task> = read_json();
    println!("This will add a new task"); //place holder while I program the actual sections

    //read the new ID and note the new ID. Do not save the new ID until the task has actually been created

    //read the existing data file
    //loop through all the items in the Task struct (except the ID) and record user input for all of them. Some can be empty but not all
    //Once done, save new list to the ddata.jsonm file and save the new ID to the count.txt file

    let count_content = fs::read_to_string("./count.txt").expect("Unable to read the file");    //reads the existing counter and parses it to an INT for manuipulation
    let mut new_id: i32 = count_content.parse().expect("Could not convert nummber to an integer"); //thi will be changed later to the new latest id and saved back to the count.txt file
    new_id +=1;
    //println!("Latest ID is: {}", new_id); //debugging
    let mut temp_inputs = Task{task_id: new_id.to_string(), ..Default::default()};

    //create a loop that runs four times. Each loop has to an input for a new variable

    for n in 1..5 {
        println!("Current Loop: {}", n);
        match n {
            1 => println!("This is for use case 1"), //make the variable we want = a function
            2 => println!("This is for use case 2"),
            3 => println!("This is for use case 3"),
            4 => println!("This is for use case 4"),
            _ => println!("This is for use case nothing")
        }
    }
}

fn get_input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");

    if s.ends_with("\n") {
        //these statements remove the newline characters at the end of the string
        s.pop();
        if s.ends_with("\r") {
            s.pop();
        }
    }
    return s;
}

fn show_tasks() {
    //used to show the tasks
    println!("TASK ID, TASK NAME, STAKE HOLDER, DUE DATE, STATE");
    let tasks: Vec<Task> = read_json();

    for lines in tasks {
        println!(
            "{} | {} | {} | {}",
            lines.task_id, lines.task_name, lines.state, lines.due_date
        );
    }
}

fn read_json() -> Vec<Task> {
    let json_file_path = Path::new("fakeData.json"); //file path of json

    let data_file: File = File::open(json_file_path).expect("File not found"); //opens the json file
                                                                               //println!("HERE");//debugging
    let tasks: Vec<Task> =
        serde_json::from_reader(data_file).expect("Error while reading data.json");
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
    let data: Vec<Task> = read_json();
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
    //This is for removing tasks from the list and saving that list over the original list
    let data: Vec<Task> = read_json(); //reads the json file and returns the struct

    println!("Please input the task ID you would like to remove (Press Q to go back)");
    let user_input = get_input(); //user input

    match user_input.as_str() {
        //checks user input to deterimine next action
        "q" => println!("Cancelled"),
        "Q" => println!("Cancelled"),
        _ => check_for_removal(&user_input, data), //Takes string and feeds it to the matching function to return a task and its details
    }
}

fn check_for_removal(input: &String, mut data: Vec<Task>) {
    let task_exists = check_if_task_exists(&input);
    let file_path: &str = "./fakeData.json";

    if task_exists == true {
        println!("Task exists and can be removed"); //if it exists, find it and return the index. then remove it
        data.remove(return_task_index(&input)); //This succsesfully deletes the task from the Vector. Needs to be rewritten back to the JSON file

        let json_converted = serde_json::to_string(&data).expect("Could not convert data to JSON");
        overwrite_existing(json_converted, file_path); //this function saves to file
    } else {
        println!("Task does not exist"); //Let's the user know the task does not exist and returns them to the main menu
    }
}

fn return_task_index(input: &String) -> usize {
    //needs to return int 32 that is the index of the task in the vector
    let data_test: Vec<Task> = read_json(); //reads the json file and returns the struct HERE FOR TESTING
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
    let data: Vec<Task> = read_json();
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

fn overwrite_existing(serialized_json: String, file_path: &str) {
    
    let mut f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    let _ = f.write_all(&serialized_json.as_bytes());
}

fn help_menu() {
    logo_print();
    println!("\nLetMeKnow Version 0.0.1 Windows Build\nThese commands have been defined internally. Type 'Help' at anytime to see this list.");
    println!("\nShow\n    Shows the user currently active tasks");
    println!("\nDetails\n     Navigates to a specific task to get additional information");
    println!("\nAdd\n     Allows the user to add an aditional task to the list");
    println!("\nEdit\n     Allows a user to edit an existing task");
    println!("\nRemove\n     Remopoves an existing task from the list");
    println!("\nExit\n     Quits the program");
}

fn logo_print() {
    //Prints the logo
    println!("          _                      _            ");
    println!("         (_)                    | |           ");
    println!("__      ___ _ __ ___  _ __    __| | _____   __");
    println!("\\ \\ /\\ / / | '__/ _ \\| '_ \\  / _` |/ _ \\ \\ / /");
    println!(" \\ V  V /| | | | (_) | | | || (_| |  __/\\ V / ");
    println!("  \\_/\\_/ |_|_|  \\___/|_| |_(_)__,_|\\___| \\_/  ");
    println!("\nWelcome to Let Me know, the task management CLI tool")
}