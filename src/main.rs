extern crate tyr;

use std::io;
use std::string::String;
use tyr::TyrError;

fn main() {
    println!("Running Tyr!");

    loop {
        println!("input command. type \"h\" for list of commands.");

        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line!");
        let command = command.trim();

        match command.as_ref() {
            "q" => break,
            "1" => {
                print_records()
            }
            "2" => {
                start_working()
            }
            "3" => {
                stop_working()
            }
            "4" => {
                write_demo_records()
            }
            "h" => {
                print_help()
            }
            _ => println!("invalid input."),
        }
    }
}

fn print_records() {
    println!("print records");
    if let Err(err) = tyr::print_records() {
        handle_error(err);
    }
}

fn start_working() {
    println!("start working");
    // TODO
}

fn stop_working() {
    println!("stop working");
    //TODO
}

fn write_demo_records() {
    println!("write demo records");
    if let Err(err) = tyr::write_demo_records() {
        handle_error(err);
    }
}

fn print_help() {
    println!("commands:");
    println!("h: print this message");
    println!("q: exit");
    println!("1: print");
    println!("2: start working");
    println!("3: stop working");
    println!("4: write demo records");
}


fn handle_error(error: TyrError) {
    println!("Error: {:?}", error);
}
