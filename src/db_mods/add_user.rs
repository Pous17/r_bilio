use std::io::{stdin, stdout, Write};
use r_bilio::*;

pub fn add_user() {
    let connection = &mut connection();

    loop {
        print!("r_bilio > add user > \n");
        stdout().flush().unwrap();

        print!("Name of the user: ");
        stdout().flush().unwrap();
        let mut user_name = String::new();
        stdin().read_line(&mut user_name).unwrap();

        print!("Does the user have a membership (y/N): ");
        stdout().flush().unwrap();
        let mut membership = String::new();
        stdin().read_line(&mut membership).unwrap();

        let choice = match membership.trim().to_lowercase().as_str() {
            "y" => true,
            "n" => false,
            _ => false,
        };

        if user_name.trim() != "" {

            let user = create_user(connection, user_name.trim_end(), &choice);
            println!("New user {} created with id {}", user.name, user.id);

            return
        } else {
            println!("Please enter valid data (empty names are not allowed)\n");
        }

    }
}