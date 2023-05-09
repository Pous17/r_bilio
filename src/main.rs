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
        
        match command {
            "add" => {
                match args {
                    "-book" => db_mods::add_book::add_book(),
                    "-user" => db_mods::add_user::add_user(),
                    "-empl" => db_mods::add_employee::add_employee(),
                    _ => println!("Unknown flag, refer to 'help'")
                }
            },
            "borrow" => {
                match args {
                    "-list" => mods::borrow_list::list(""),
                    "-list-id" => mods::borrow_list::list("id"),
                    "-list-date" => mods::borrow_list::list("date"),
                    "-list-id-date" => mods::borrow_list::list("id-date"),
                    "" => db_mods::borrow_book::borrow_book(),
                    _ => println!("Unknown flag, refer to 'help'")
                }
            },
            "return" => db_mods::return_book::return_book(),
            "status" => {
                match args {
                    "-book" => mods::status::status("book"),
                    "-user" => mods::status::status("user"),
                    "-empl" => mods::status::status("empl"),
                    "-book-id" => mods::status::status("book -id"),
                    "-user-id" => mods::status::status("user -id"),
                    "-empl-id" => mods::status::status("empl -id"),
                    "-id" => mods::status::status("all -id"),
                    "-info" => mods::status::status("all -info"),
                    "-logs" => mods::status::status("logs"),
                    _ => mods::status::status("all")
                }
            },
            "populate" => db_mods::populate::populate(),
            "help" => mods::help::help(),
            "exit" => return,
            "" => {},
            _ => println!("Unknown command, enter 'help'\n")
        }
    }
}