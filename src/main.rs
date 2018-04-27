extern crate chrono;
extern crate tyr;

use chrono::prelude::*;
use std::io;
use std::string::String;

fn main() {
    println!("Running Tyr!");

    loop {
        println!("input command. type \"h\" for list of commands.");

        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line!");
        let command = command.trim();

        match command.as_ref() {
            "0" => break,
            "1" => {
                println!("reading...");
                if let Err(err) = tyr::read_csv() {
                    println!("Error: {}", err);
                }
            }
            "2" => {
                println!("writing...");
                if let Err(err) = tyr::write_csv() {
                    println!("Error: {}", err);
                }
            }
            "3" => {
                let now = Utc::now();
                let now = now.with_second(0).unwrap();
                let now = now.with_nanosecond(0).unwrap();
                println!("now: {:?}", now);
                let t = Utc.ymd(2018, 4, 27).and_hms(10, 50, 0);
                println!("time: {:?}", t);
            }
            "h" => {
                println!("commands:");
                println!("h: print this message");
                println!("0: exit");
                println!("1: read");
                println!("2: write");
            }
            _ => println!("invalid input."),
        }
    }
}
