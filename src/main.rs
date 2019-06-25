extern crate chrono;
extern crate tyr;
#[macro_use] extern crate log;
extern crate env_logger;

use std::io;
use std::string::String;

use chrono::Utc;

use tyr::TyrError;

fn main() {
    println!("Running Tyr!");
    env_logger::init();

    loop {
        println!("Enter command. Type \"h\" for list of commands.");

        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line!");
        let command = command.trim();

        match command.as_ref() {
            "q" => break,
            "1" => {
                print_records()
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
    trace!("print_records()");
    if let Err(err) = tyr::print_records() {
        handle_error(err)
    }
}

fn start_working() -> Result<(), TyrError> {
    trace!("start_working()");

    let time = Utc::now();
    tyr::start_progress(time, "Foobar".to_string())?;
    Ok(())
}

fn stop_working() -> Result<(), TyrError> {
    trace!("stop_working()");

    let time = Utc::now();
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
    println!("1: print");
    println!("2: start working");
    println!("3: stop working");
    println!("4: write demo records");
}


fn handle_error(error: TyrError) {
    println!("Error: {:?}", error);
}
