use std::{env, process::Command};
use terminal_size::*;

//  This crate enables you to have colored strings. For example: println!({}, "This is green.".green()); Check this crate out, so you can understand it better:
//  https://github.com/colored-rs/colored
pub use colored::*;

use crate::gen1cmds;

//  This prints out a list of strings with the built in println!() Rust macro.
pub fn print_list(strings: Vec<&str>, color: Option<Color>) {
    for string in strings {
        match color {
            Some(existing_color) => println!("{}", string.color(existing_color)),
            None => println!("{}", string)
        };
    }
}

//  This is almost the same as the to_bool function in the gen1cmds, but this time, it is a really common way to ask someone a yes or no question. Returns a bool (y = true, n = false).
pub fn y_or_n(color: Option<Color>) -> bool {
    match color {
        Some(existing_color) => println!("{}", "y/n?".color(existing_color)),
        None => println!("y/n?")
    }
    let mut input;
    loop {
        gen1cmds::readln!(input);
        gen1cmds::endl!();
        if input.trim() == "y" {
            return true;
        } else if input.trim() == "n" {
            return false;
        } else {
            match color {
                Some(_) => println!("{}", "Invalid option (y/n)".red()),
                None => println!("Invalid option (y/n)")
            }
        }
    }
}

//  This function returns the OS name as a string. Like "windows", "linux" or "macos".
pub fn get_current_os() -> String {
    return env::consts::OS.to_string();
}

//  This function clears the terminal screen the way it should for each OS.
pub fn clear_terminal() {
    match get_current_os().as_str() {
        "windows" => {
            Command::new("cls").status().unwrap();
        },
        "linux" => {
            Command::new("clear").status().unwrap();
        },
        "macos" => {
            Command::new("clear").status().unwrap();
        },
        _ => {
            Command::new("clear").status().unwrap();
        }
    }
}

//  Prints a string in the middle of char_len.
pub fn middle_print(text: &str, char_len: usize) {
    print!("{:^char_len$}", text);
}

//  Prints a string in the center of the terminal screen.
pub fn centered_println(text: &str) {
    let term_size = terminal_size();
    if let Some((Width(w), Height(_))) = term_size {
        let width = w as usize;
        println!("{:^width$}", text);
    }
}

//  This function doesn't do anything. This is useful for (as an example) the match keyword, when you have to do something for every possible pattern.
pub fn pass() {}