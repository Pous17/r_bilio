use std::io::{stdin, stdout, Write};
use std::env;
use dotenvy::dotenv;
use console::Term;
use crypto_hash::{Algorithm, hex_digest};
use std::process::Command;

mod mods;
mod db_mods;

fn main() {
    dotenv().ok();

    let env_pass: String = env::var("HASH_MDP").expect("No password has been set");
    let _hash_env_pass = hex_digest(Algorithm::SHA256, env_pass.as_bytes());
    let term = Term::stdout();
    let mut run = true;
    let mut hash_pass: String;
    let mut trimmed_hash_pass: &str;
    let mut _role: &str;

    loop {
        Command::new("clear").status().unwrap();
        
        _role = "";

        println!("\n---------- R Bilio Manager ----------\n");
        println!("Enter 'help' for help.");
        println!("\nIf you are a user, just press enter");

        // Password loop
        loop {
            print!("Password: ");
            stdout().flush().unwrap();

            hash_pass = hex_digest(Algorithm::SHA256, term.read_secure_line().unwrap().as_bytes());
            trimmed_hash_pass = hash_pass.trim();

            match trimmed_hash_pass {
                pass if pass == _hash_env_pass => {
                    println!("You are loged as an admin");
                    _role = "admin";
                }
                "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855" => { // empty string
                    println!("You are loged as a user");
                    _role = "user";
                }
                _ => {
                    println!("Wrong password. Retry.");
                }
            }
            
            if _role != "" {
                break
            }
        }
    
        // Command loop
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
                (_, "status", "-book") => mods::status::status("book"),
                (_, "status", "-book-id") => mods::status::status("book -id"),
                (_, "help", "") => mods::help::help(_role),
                (_, "exit" | "logout", "") => break,
                (_, "terminate", "process") => {
                    run = false;
                    break
                },
                (_, "", "") => {},
                _ => println!("Unknown command or unauthorized access"),
            }
        }

        if !run {
            break
        }

    }
}