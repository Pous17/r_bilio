use std::io::{stdin, stdout, Write};
use r_bilio::*;

use super::fetch_db::fetch;

pub fn update_member() {
    let lists = fetch();
    let user_list = lists.1;

    let connection = &mut connection();

    println!("List of members: ");
    println!("--------------");
    for user in &user_list {
        if user.member {
            println!("{} | id: {}", user.name, user.id);
        }
    }

    loop {
        println!("\nr_bilio > update membership > ");
        print!("Id of the member you want to change membership: ");
        stdout().flush().unwrap();

        let mut user_input = String::new();
        stdin().read_line(&mut user_input).unwrap();

        print!("Is the user a member now (y/N): ");
        stdout().flush().unwrap();

        let mut membership = String::new();
        stdin().read_line(&mut membership).unwrap();

        let choice = match membership.trim().to_lowercase().as_str() {
            "y" => true,
            "n" => false,
            _ => false,
        };

        // i32 parsing
        let user_id = user_input.trim_end().parse::<i32>().unwrap_or(-1);

        if user_id == -1 {
            println!("Enter a valid number");
        } else {
            update_membership(connection, &user_id, &choice);
            return
        }
    }
}