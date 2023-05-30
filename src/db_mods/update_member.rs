use r_bilio::*;

use crate::{db_mods::fetch_db::fetch_users, utils::{input_string, input_i32}};


pub fn update_member(login: &str, str_date: &str) {
    let connection = &mut connection();
    let users_list = fetch_users();

    println!("List of members: ");
    println!("--------------");
    for user in &users_list {
        if user.member {
            println!("{} {} | id: {}", user.firstname, user.lastname, user.id);
        }
    }

    loop {
        println!("\nr_bilio > update membership > ");

        let user_id = input_i32("Id of the member you want to change membership: ");
        let str_membership = input_string("Is the user a member now (Y/n): ");

        let membership = match str_membership.trim().to_lowercase().as_str() {
            "y" => true,
            "n" => false,
            _ => true,
        };

        if user_id < 0 {
            println!("Enter a valid Id number");
        } else {
            if let Some(user) = users_list.iter().find(|user| user.id == user_id) {
                let text = match membership {
                    true => "is now a member",
                    false => "is no longer a member"
                };

                println!("{} {} {}\n", user.firstname, user.lastname, text);
                
                update_membership(
                    connection,
                    &user.id,
                    &membership,
                    login,
                    str_date,
                );

                return
            } else {
                println!("There is no user for the id you provided");
            }
        }
    }
}
