extern crate tyr;
extern crate chrono;

use std::io;
use std::string::String;

use tyr::TyrError;
use chrono::Utc;

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
                if let Err(err) = start_working() {
                    handle_error(err)
                }
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
        handle_error(err)
    }
}

fn start_working() -> Result<(), TyrError>{
    println!("start working");

    let latest = tyr::get_latest_record()?;
    match latest {
        Some(r) => {
            let time = Utc::now();
            tyr::stop_progress(time);
        },
        _ => ()
    }

    // TODO


    Ok(())
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
