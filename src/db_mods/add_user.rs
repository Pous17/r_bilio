use r_bilio::*;

use crate::{db_mods::fetch_db::fetch_users, utils::{get_name, input_string, name_check, get_password}};


pub fn add_user(login: &str, str_date: &str) {
    loop {
        println!("r_bilio > add user >");
        let (user_firstname, user_lastname) = get_name("Name of the user: ");
        let membership = input_string("Does the user have a membership (Y/n): ").to_lowercase();

        let has_membership = match membership.as_str() {
            "y" => true,
            "n" => false,
            _ => true,
        };

        if name_check(&user_firstname, &user_lastname, false) {
            let connection = &mut connection();
            let users_list = fetch_users();

            if let Some(user) = users_list.iter()
            .find(|x| x.firstname == user_firstname && x.lastname == user_lastname) {
                print!("{} {} already exists.", user.firstname, user.lastname);
                return;
            }

            let hash_pass = get_password(None);
            
            let user = create_user(
                connection, 
                has_membership,
                &user_firstname,
                &user_lastname,
                &hash_pass,
                login,
                str_date,
            );

            println!("New user {} {} created \nwith id {} and login {}\n",
            user.firstname, user.lastname, user.id, user.login);
            return;
        }
    }
}

