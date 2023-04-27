use std::io::{stdin, stdout, Write};
// use std::process::{Command};

mod mods;
mod db_mods;
fn main() {

    println!("\n---------- R Bilio Manager ----------\n");

    loop {
        print!("r_bilio > ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap_or("");
        let args = parts.last().unwrap_or("");

        // println!("{}", command);
        // println!("{:?}", args);
        
        match command {
            // "bilio_add_book" => db_mods::add_book::add_book(),
            "add" => {
                match args {
                    "-book" => db_mods::add_book::add_book(),
                    "-user" => {}, // add user
                    "-empl" => {}, // add empl
                    _ => println!("Invalid flag, refer to 'help'")
                }
            }
            "help" => mods::help::help(),
            "status" => mods::status::status(),
            "exit" => return,
            "" => {},
            _ => println!("Unkonwed command, enter 'bilio_help'\n")
        }
    }
}