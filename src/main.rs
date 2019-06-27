extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate tyr;

use std::io;
use std::string::String;

use chrono::prelude::*;
use chrono::Utc;

use tyr::TyrError;

fn main() {
    println!("Running Tyr!");
    env_logger::init();

    loop {
        println!("Enter command. Type \"h\" for list of commands.");

        let command = read_input();

        match command.as_ref() {
            "q" => break,
            "1" => {
                print_times()
            }
            "2" => {
                if let Err(err) = start_working() {
                    handle_error(err)
                }
            }
            "3" => {
                if let Err(err) = stop_working() {
                    handle_error(err)
                }
            }
            "8" => {
                print_records()
            }
            "9" => {
                write_demo_records()
            }
            "h" => {
                print_help()
            }
            _ => println!("invalid input."),
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    input.trim().to_string()
}

fn print_times() {
    trace!("print_times()");
    if let Err(err) = tyr::print_times() {
        handle_error(err)
    }
}

fn print_records() {
    if let Err(err) = tyr::print_records() {
        handle_error(err)
    }
}

fn start_working() -> Result<(), TyrError> {
    trace!("start_working()");

    let time = Utc::now().with_second(0).unwrap().with_nanosecond(0).unwrap();
    println!("What are you working on?");
    let title = read_input();
    tyr::start_progress(time, title)?;
    Ok(())
}

fn stop_working() -> Result<(), TyrError> {
    trace!("stop_working()");

    let time = Utc::now().with_second(0).unwrap().with_nanosecond(0).unwrap();
    let result = tyr::stop_progress(time)?;
    if result == false {
        println!("You are not currently working on anything.")
    }
    Ok(())
}

fn write_demo_records() {
    trace!("write_demo_records()");
    if let Err(err) = tyr::write_demo_records() {
        handle_error(err);
    }
}

fn print_help() {
    println!("commands:");
    println!("h: print this message");
    println!("q: exit");
    println!("1: print times");
    println!("2: start working");
    println!("3: stop working");
    println!("8: print raw records");
    println!("9: write demo records");
}


fn handle_error(error: TyrError) {
    println!("Error: {:?}", error);
}
