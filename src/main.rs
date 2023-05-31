use std::io::{stdin, stdout, Write};
use chrono::{Local, Duration};
use dotenvy::dotenv;
use console::Term;
use crypto_hash::{Algorithm, hex_digest};
use std::process::Command;

use crate::utils::input_string;

mod mods;
mod db_mods;
mod utils;


fn main() {
    dotenv().ok();
    
    let term = Term::stdout();
    let mut run = true;
    let mut hash_pass: String;
    let mut trimmed_hash_pass: &str;
    let mut _login: String;
    let mut _role: &str;

    let date = Local::now();
    let limit_date = date + Duration::days(7);
    let str_date = date.format("%Y-%m-%d").to_string();
    let str_limit_date = limit_date.format("%Y-%m-%d").to_string();

    loop {
        let accounts_list = db_mods::fetch_db::fetch_accounts();
        let users_list = accounts_list.0;
        let employees_list = accounts_list.1;

        _login = "".to_owned();
        _role = "";

        Command::new("clear").status().unwrap();

        println!("\n---------- R Bilio Manager ----------\n");
        println!("Enter 'help' for help.");
        println!("\nIf you don't have an account, just press Enter");

        // Login loop
        loop {
            _login = input_string("Login: ");

            if _login == "" {
                _login = "user".to_owned();
                _role = "user";
                println!("You are logged as a user");
                break
            }

            print!("Password: ");
            stdout().flush().unwrap();
            hash_pass = hex_digest(Algorithm::SHA256, term.read_secure_line().unwrap().as_bytes());
            trimmed_hash_pass = hash_pass.trim();

            if let Some(employee) = employees_list.iter().find(|employee: &&r_bilio::models::Employee| employee.login == _login) {
                match trimmed_hash_pass {
                    _pass if trimmed_hash_pass == employee.password => {
                        println!("You are logged as {} {}", employee.firstname, employee.lastname);
                        _login = employee.login.to_owned();
                        _role = &employee.role;
                    },
                    _ => {
                        println!("Wrong password. Retry.");
                    }
                }
            } else if let Some(user) = users_list.iter().find(|user: &&r_bilio::models::User| user.login == _login) {
                match trimmed_hash_pass {
                    _pass if trimmed_hash_pass == user.password => {
                        println!("You are logged as {} {}", user.firstname, user.lastname);
                        _login = user.login.clone();
                        _role = &user.role;
                    },
                    _ => {
                        println!("Wrong password. Retry.");
                    }
                }
            } else {
                println!("Unknown login. Retry.");
            }

            if _role != "" {
                break
            }
        }
    
        // Command loop
        loop {
            print!("{} > r_bilio > ", _login);
            stdout().flush().unwrap();

            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            input = input.trim().to_owned().to_lowercase();
            let mut parts = input.trim().split_whitespace();
            let command = parts.next().unwrap_or("");
            let args = parts.last().unwrap_or("");

            match (_role, command, args) {

                // Admin only
                ("employee", "add", "-book") => db_mods::add_book::add_book(&_login, &str_date),
                ("employee", "add", "-user") => db_mods::add_user::add_user(&_login, &str_date),
                ("employee", "add", "-empl") => db_mods::add_employee::add_employee(&_login, &str_date),
                ("employee", "borrow", "-list") => mods::borrow_list::list(""),
                ("employee", "borrow", "-list-id") => mods::borrow_list::list("id"),
                ("employee", "borrow", "-list-date") => mods::borrow_list::list("date"),
                ("employee", "borrow", "-list-id-date") => mods::borrow_list::list("id-date"),
                ("employee", "borrow", "-list-user") => mods::user_borrows::user_borrows(),
                ("employee", "borrow", "") => db_mods::borrow_book::borrow_book(&_login, &str_date, &str_limit_date),
                ("employee", "return", _) => db_mods::return_book::return_book(&_login, &str_date),
                ("employee", "status", "") => mods::status::status("all"),
                ("employee", "status", "-id") => mods::status::status("all -id"),
                ("employee", "status", "-info") => mods::status::status("all -info"),
                ("employee", "status", "-user") => mods::status::status("user"),
                ("employee", "status", "-user-id") => mods::status::status("user -id"),
                ("employee", "status", "-user-info") => mods::status::status("user -info"),
                ("employee", "status", "-empl") => mods::status::status("empl"),
                ("employee", "status", "-empl-id") => mods::status::status("empl -id"),
                ("employee", "status", "-empl-info") => mods::status::status("empl -info"),
                ("employee", "status", "-borrow") => mods::status::status("borrow"),
                ("employee", "status", "-borrow-id") => mods::status::status("borrow -id"),
                ("employee", "status", "-borrow-info") => mods::status::status("borrow -info"),
                ("employee", "status", "-pastborrow") => mods::status::status("past"),
                ("employee", "status", "-pastborrow-id") => mods::status::status("past -id"),
                ("employee", "status", "-pastborrow-info") => mods::status::status("past -info"),
                ("employee", "update", "-member") => db_mods::update_member::update_member(&_login, &str_date),
                ("employee", "populate", "") => db_mods::populate::populate(&_login, &str_date),

                // User authorized
                (_, "status", "-author") => mods::status::status("author"),
                (_, "status", "-author-id") => mods::status::status("author -id"),
                (_, "status", "-book") => mods::status::status("book"),
                (_, "status", "-book-id") => mods::status::status("book -id"),
                (_, "status", "-book-info") => mods::status::status("book -info"),
                (_, "author", "-book") => mods::author_books_list::author_books(),
                (_, "update", "-password") => db_mods::update_password::update_pass(&_role, &_login, &str_date),
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