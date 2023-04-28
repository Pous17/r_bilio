use std::io::{stdin, stdout, Write};
// use std::process::{Command};

mod mods;
mod db_mods;
fn main() {
    println!("\n---------- R Bilio Manager ----------\n");
    println!("Enter 'help' for help.");

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
        
        match command.trim_end() {
            "add" => {
                match args {
                    "-book" => db_mods::add_book::add_book(),
                    "-user" => db_mods::add_user::add_user(),
                    "-empl" => {}, // add empl
                    _ => println!("Unknown flag, refer to 'help'")
                }
            },
            "borrow" => {
                match args {
                    "-list" => mods::borrow_list::list(""),
                    "-list-id" => mods::borrow_list::list("id"),
                    "" => db_mods::borrow_book::borrow_book(),
                    _ => println!("Unknown flag, refer to 'help'")
                }
            },
            "help" => mods::help::help(),
            "status" => {
                match args {
                    "-id" => mods::status::status("id"),
                    _ => mods::status::status("")
                }
            }
            "exit" => return,
            "" => {},
            _ => println!("Unknown command, enter 'bilio_help'\n")
        }
    }
}