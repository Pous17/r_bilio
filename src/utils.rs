use std::io::{stdout, stdin, Write};

use console::Term;
use crypto_hash::{hex_digest, Algorithm};
use regex::Regex;

pub fn input_i32(prompt: &str) -> i32 {
    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(-1)
}

pub fn input_string(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn get_name(prompt: &str) -> (&str, &str) {
    let name = input_string(prompt);
    let firstname = name.split(" ").next().unwrap().trim();
    let lastname = name.split(" ").last().unwrap().trim();
    (firstname, lastname)
}

pub fn get_password() -> &'static str {
    print!("Password: ");
    stdout().flush().unwrap();
    let term = Term::stdout();
    let hash_pass = hex_digest(Algorithm::SHA256, term.read_secure_line().unwrap().as_bytes());
    hash_pass.trim()
}

pub fn name_check(firstname: &str, lastname: &str, author: bool) -> bool {
    let pattern = Regex::new(r"^[a-zA-Z]{1,}$").unwrap();

    if (firstname.is_empty() && !author) || lastname.is_empty() {
        println!("Please enter valid data (empty names are not allowed)\n");
        false
    } else if (!pattern.is_match(firstname) && !author) || !pattern.is_match(lastname) {
        println!("Please enter valid data (firstname and lastname can only contains letters)\n");
        false
    } else if firstname == lastname {
        println!("Please enter valid data (firstname and lastname can't be the same)\n");
        false
    } else {
        true
    }
}
