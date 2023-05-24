use std::io::{stdin, stdout, Write};
use r_bilio::*;

pub fn add_user() {
    let connection = &mut connection();

    loop {
        println!("r_bilio > add user >");

        let user_name = input_string("Name of the user: ");
        let membership = input_string("Does the user have a membership (y/N): ").to_lowercase();

        let has_membership = match membership.as_str() {
            "y" => true,
            "n" => false,
            _ => false,
        };

        if !user_name.is_empty() {
            let user = create_user(connection, &user_name, &has_membership);
            println!("New user {} created with id {}\n", user.name, user.id);
            return;
        } else {
            println!("Please enter valid data (empty names are not allowed)\n");
        }
    }
}

fn input_string(prompt: &str) -> String {
    print!("{}", prompt);
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}