extern crate tyr;

use std::io;
use std::string::String;

fn main() {
    println!("Running Tyr!");

    loop {
        println!("input command.");
        println!("1: read");
        println!("2; write");
        println!("0: exit");

        let mut command = String::new();

        io::stdin().read_line(&mut command).expect("Failed to read line!");
        let command = command.trim();
        println!("command: {}", &command);

        match command.as_ref() {
            "0" => {
                break;
            }
            "1" => {
                println!("reading...")
            }
            "2" => {
                println!("writing...")
                if let Err(err) = tyr::write_csv() {
                    println!("Error: {}", err);
                }
            }
            _ => {
                println!("invalid input.")
            }
        }
    }
}
