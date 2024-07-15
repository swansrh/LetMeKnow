use std::io;

fn main() {
    println!("Hello, world!");
    logo_print();
    main_menu()
}

fn logo_print() {
    println!("          _                      _            ");
    println!("         (_)                    | |           ");
    println!("__      ___ _ __ ___  _ __    __| | _____   __");
    println!("\\ \\ /\\ / / | '__/ _ \\| '_ \\  / _` |/ _ \\ \\ / /");
    println!(" \\ V  V /| | | | (_) | | | || (_| |  __/\\ V / ");
    println!("  \\_/\\_/ |_|_|  \\___/|_| |_(_)__,_|\\___| \\_/  ");
    println!("\nWelcome to Let Me know, the task management CLI tool")
}

fn main_menu() {
    let mut user_input = String::new(); //used for input
    println!("\nPlease see pending tasks below:\n");
    show_tasks();
    println!("\nWhat would you like to do? (Type 'Help' for help)");

    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    println!("{}", user_input)
}

fn show_tasks() {//used to show the tasks
    for i in 1..10{
        println!("SICK TASK NUMBER: {}", i);
    }
}

fn help_menu() {
    println!("")
}