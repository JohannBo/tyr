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
            "q" | "quit" | "exit" => {
                println!("Bye!");
                break;
            }
            "l" | "list" => {
                print_times()
            }
            "s" | "start" => {
                if let Err(err) = start_working() {
                    handle_error(err)
                }
            }
            "t" | "stop" => {
                if let Err(err) = stop_working() {
                    handle_error(err)
                }
            }
            "p" | "pause" => {
                if let Err(err) = pause_working() {
                    handle_error(err)
                }
            }
            "raw" => {
                print_records()
            }
            "demo" => {
                write_demo_records()
            }
            "h" | "help" => {
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
    trace!("print_records()");

    if let Err(err) = tyr::print_records() {
        handle_error(err)
    }
}

fn start_working() -> Result<(), TyrError> {
    trace!("start_working()");

    println!("What are you working on?");
    let title = read_input();
    println!("When did you start working?");
    println!("Example: <empty>/\"now\"/'0' -> now; 5 -> five minutes ago; -5 -> in five minutes");
    let time = read_time_with_offset();

    tyr::start_progress(time, title)?;
    Ok(())
}

fn stop_working() -> Result<(), TyrError> {
    trace!("stop_working()");

    println!("When did you stop working?");
    println!("Example: <empty>/\"now\"/'0' -> now; 5 -> five minutes ago; -5 -> in five minutes");
    let time = read_time_with_offset();
    let result = tyr::stop_progress(time)?;
    if result == false {
        println!("You are not currently working on anything.")
    }
    Ok(())
}

fn pause_working() -> Result<(), TyrError> {
    trace!("pause_working()");

    println!("How long was your pause? (in minutes)");
    let minutes;
    loop {
        let command = read_input();
        minutes = match command.parse::<i64>() {
            Ok(i) => i,
            Err(_) => {
                println!("Invalid input. \"{}\" cannot be converted to integer.", command);
                continue;
            }
        };
        break;
    }
    let stop_time = Utc::now().with_second(0).unwrap().with_nanosecond(0).unwrap();
    let start_time = stop_time - Duration::minutes(minutes);

    tyr::pause_progress(start_time, stop_time)
}

fn write_demo_records() {
    trace!("write_demo_records()");

    if let Err(err) = tyr::write_demo_records() {
        handle_error(err);
    }
}

fn print_help() {
    trace!("print_help()");

    println!("commands:");
    println!("h / help: print this message");
    println!("q / quit / exit: exit");
    println!("l / list: print times");
    println!("s / start: start working");
    println!("t / stop: stop working");
    println!("p / pause: pause then continue");
    println!("raw: print raw records");
    println!("demo: write demo records");
}


fn handle_error(error: TyrError) {
    trace!("handle_error()");

    println!("Error: {:?}", error);
}
