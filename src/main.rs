use std::io::{stdin, stdout, Write};
use dotenvy::dotenv;
use console::Term;
use crypto_hash::{Algorithm, hex_digest};
use std::process::Command;

mod mods;
mod db_mods;

fn main() {
    dotenv().ok();
    
    let term = Term::stdout();
    let mut run = true;
    let mut hash_pass: String;
    let mut trimmed_hash_pass: &str;
    let mut _login: &str;
    let mut _role: &str;

    loop {
        let accounts_list = db_mods::fetch_db::fetch_accounts();
        let users_list = accounts_list.0;
        let employees_list = accounts_list.1;

        _login = "";
        _role = "";

        Command::new("clear").status().unwrap();

        println!("\n---------- R Bilio Manager ----------\n");
        println!("Enter 'help' for help.");
        println!("\nIf you don't have an account, just press Enter");

        // Login loop
        loop {
            print!("Login: ");
            stdout().flush().unwrap();
            _login = term.read_line().unwrap().as_str();

            if _login == "" {
                _login = "user";
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
                    pass if trimmed_hash_pass == employee.password => {
                        println!("You are logged as {} {}", employee.firstname, employee.lastname);
                        _login = &employee.login;
                        _role = &employee.role;
                    },
                    _ => {
                        println!("Wrong password. Retry.");
                    }
                }
            } else if let Some(user) = users_list.iter().find(|user: &&r_bilio::models::User| user.login == _login) {
                match trimmed_hash_pass {
                    pass if trimmed_hash_pass == user.password => {
                        println!("You are logged as {} {}", user.firstname, user.lastname);
                        _login = &user.login;
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

            let mut parts = input.trim().split_whitespace();
            let command = parts.next().unwrap_or("");
            let args = parts.last().unwrap_or("");

            match (_role, command, args) {

                // Admin only
                ("employee", "add", "-book") => db_mods::add_book::add_book(),
                ("employee", "add", "-user") => db_mods::add_user::add_user(),
                ("employee", "add", "-empl") => db_mods::add_employee::add_employee(),
                ("employee", "borrow", "-list") => mods::borrow_list::list(""),
                ("employee", "borrow", "-list-id") => mods::borrow_list::list("id"),
                ("employee", "borrow", "-list-date") => mods::borrow_list::list("date"),
                ("employee", "borrow", "-list-id-date") => mods::borrow_list::list("id-date"),
                ("employee", "borrow", "-list-user") => mods::user_borrows::user_borrows(),
                ("employee", "borrow", "") => db_mods::borrow_book::borrow_book(),
                ("employee", "return", _) => db_mods::return_book::return_book(),
                ("employee", "status", "-user") => mods::status::status("user"),
                ("employee", "status", "-empl") => mods::status::status("empl"),
                ("employee", "status", "-user-id") => mods::status::status("user -id"),
                ("employee", "status", "-empl-id") => mods::status::status("empl -id"),
                ("employee", "status", "-id") => mods::status::status("all -id"),
                ("employee", "status", "-info") => mods::status::status("all -info"),
                ("employee", "status", "-logs") => mods::status::status("logs"),
                ("employee", "status", "") => mods::status::status("all"),
                ("employee", "update", "-member") => db_mods::update_member::update_member(),
                ("employee", "populate", "") => db_mods::populate::populate(),

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