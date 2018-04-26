extern crate tyr;

use std::io;
use std::string::String;

fn main() {
    println!("Running Tyr!");

    loop {
        println!("input command. type \"h\" for list of commands.");

        let mut command = String::new();

        io::stdin().read_line(&mut command).expect("Failed to read line!");
        let command = command.trim();
        println!("command: {}", &command);

        match command.as_ref() {
            "0" => {
                break;
            }
            "1" => {
                println!("reading...");
            }
            "2" => {
                println!("writing...");
                if let Err(err) = tyr::write_csv() {
                    println!("Error: {}", err);
                }
            }
            "h" => {
                println!("commands:");
                println!("0: exit");
                println!("1: read");
                println!("2: write");
            }
            _ => {
                println!("invalid input.");
            }
        }
    }
}
