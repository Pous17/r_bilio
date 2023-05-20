use std::io::{stdin, stdout, Write};
use console::Term;
// use std::process::{Command};

mod mods;
mod db_mods;
fn main() {
    println!("\n---------- R Bilio Manager ----------\n");
    println!("Enter 'help' for help.");

    let term = Term::stdout();

    println!("\nIf you are a user, just press enter");
    print!("Role: ");
    stdout().flush().unwrap();
    let pass = term.read_secure_line().unwrap();

    let mut _role = "";
    match pass.trim() {
        "passroot" => {
            println!("You are loged as an admin\n");
            _role = "admin";
        },
        _ => {
            println!("You are loged as a user\n");
            _role = "user";
        }
    }

    loop {
        print!("r_bilio > ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap_or("");
        let args = parts.last().unwrap_or("");

        match (_role, command, args) {

            // Admin only
            ("admin", "add", "-book") => db_mods::add_book::add_book(),
            ("admin", "add", "-user") => db_mods::add_user::add_user(),
            ("admin", "add", "-empl") => db_mods::add_employee::add_employee(),
            ("admin", "borrow", "-list") => mods::borrow_list::list(""),
            ("admin", "borrow", "-list-id") => mods::borrow_list::list("id"),
            ("admin", "borrow", "-list-date") => mods::borrow_list::list("date"),
            ("admin", "borrow", "-list-id-date") => mods::borrow_list::list("id-date"),
            ("admin", "borrow", "-list-user") => mods::user_borrows::user_borrows(),
            ("admin", "borrow", "") => db_mods::borrow_book::borrow_book(),
            ("admin", "return", _) => db_mods::return_book::return_book(),
            ("admin", "status", "-user") => mods::status::status("user"),
            ("admin", "status", "-empl") => mods::status::status("empl"),
            ("admin", "status", "-user-id") => mods::status::status("user -id"),
            ("admin", "status", "-empl-id") => mods::status::status("empl -id"),
            ("admin", "status", "-id") => mods::status::status("all -id"),
            ("admin", "status", "-info") => mods::status::status("all -info"),
            ("admin", "status", "-logs") => mods::status::status("logs"),
            ("admin", "status", "") => mods::status::status("all"),
            ("admin", "update", "-member") => db_mods::update_member::update_member(),
            ("admin", "populate", "") => db_mods::populate::populate(),

            // User authorized
            ("admin" | "user", "status", "-book") => mods::status::status("book"),
            ("admin" | "user", "status", "-book-id") => mods::status::status("book -id"),
            ("admin" | "user", "help", "") => mods::help::help(_role),
            ("admin" | "user", "exit", "") => return,
            (_, "", "") => {},
            _ => println!("Unknown command or unauthorized access"),
        }
        
        // match command {
        //     "add" => {
        //         match args {
        //             "-book" => db_mods::add_book::add_book(),
        //             "-user" => db_mods::add_user::add_user(),
        //             "-empl" => db_mods::add_employee::add_employee(),
        //             _ => println!("Unknown flag '{}' for '{}', refer to 'help'", args, command)
        //         }
        //     },
        //     "borrow" => {
        //         match args {
        //             "-list" => mods::borrow_list::list(""),
        //             "-list-id" => mods::borrow_list::list("id"),
        //             "-list-date" => mods::borrow_list::list("date"),
        //             "-list-id-date" => mods::borrow_list::list("id-date"),
        //             "-user" => mods::user_borrows::user_borrows(),
        //             "" => db_mods::borrow_book::borrow_book(),
        //             _ => println!("Unknown flag '{}' for '{}', refer to 'help'", args, command)
        //         }
        //     },
        //     "return" => db_mods::return_book::return_book(),
        //     "status" => {
        //         match args {
        //             "-book" => mods::status::status("book"),
        //             "-user" => mods::status::status("user"),
        //             "-empl" => mods::status::status("empl"),
        //             "-book-id" => mods::status::status("book -id"),
        //             "-user-id" => mods::status::status("user -id"),
        //             "-empl-id" => mods::status::status("empl -id"),
        //             "-id" => mods::status::status("all -id"),
        //             "-info" => mods::status::status("all -info"),
        //             "-logs" => mods::status::status("logs"),
        //             "" => mods::status::status("all"),
        //             _ => println!("Unknown flag '{}' for '{}', refer to 'help'", args, command)
        //         }
        //     },
        //     "update" => {
        //         match args {
        //             "-member" => db_mods::update_member::update_member(),
        //             _ => println!("Unknown flag '{}' for '{}', refer to 'help'", args, command)
        //         }
        //     }
        //     "populate" => db_mods::populate::populate(),
        //     "help" => mods::help::help(),
        //     "exit" => return,
        //     "" => {},
        //     _ => println!("Unknown command '{}', enter 'help'\n", command)
        // }
    }
}