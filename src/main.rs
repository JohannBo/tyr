extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate tyr;

use std::io;
use std::string::String;

use chrono::Duration;
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
            "4" => {
                if let Err(err) = pause_working() {
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
    trace!("read_input()");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!");
    input.trim().to_string()
}

fn read_time_with_offset() -> DateTime<Utc> {
    trace!("read_time_with_offset()");
    loop {
        let command = read_input();
        let minutes;

        if command.is_empty() || command == "now".to_string() {
            minutes = 0;
        } else {
            minutes = match command.parse::<i64>() {
                Ok(i) => i,
                Err(_) => {
                    println!("Invalid input. \"{}\" cannot be converted to integer.", command);
                    continue;
                }
            };
        }
        return Utc::now().with_second(0).unwrap().with_nanosecond(0).unwrap() - Duration::minutes(minutes);
    }
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

    println!("What are you working on?");
    let title = read_input();
    println!("When did you start? (<empty>/\"now\"/'0' -> now; 5 -> five minutes ago; -5 -> in five minutes");
    let time = read_time_with_offset();

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

fn pause_working() -> Result<(), TyrError> {
    trace!("pause_working()");
    //TODO
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
