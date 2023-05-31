use r_bilio::*;

use crate::{db_mods::fetch_db::fetch_accounts,utils::{get_password, input_string}};


pub fn update_pass(role: &str, login: &str, str_date: &str) {
    let connection = &mut connection();
    let (users_list, employees_list) = fetch_accounts();

    let mut _account_login: String = String::new();
    let mut _old_password: String = String::new();

    if role == "employee" {
        let account_choice = input_string("Do you want to update your own password ? (Y/n): ");

        match account_choice.to_lowercase().as_str() {
            "y" => (),
            "n" => {
                _account_login = input_string("Enter the login of the user you want to update: ").to_lowercase();
            },
            _ => (),
        };
    }

    if _account_login.is_empty() {
        _account_login = login.to_string();
        _old_password = get_password(Some("Old password: "));
    }

    if let Some(user) = users_list.iter().find(|x| x.login == _account_login) {
        if role == "employee" || user.password == _old_password {
            let new_password = get_password(Some("New password: "));
            let new_password_confirm = get_password(Some("Confirm new password: "));

            if new_password == new_password_confirm {
                update_password(
                    connection,
                    &user.id,
                    &new_password,
                    true,
                    login,
                    str_date
                );

                println!("Password updated successfully");
            } else {
                println!("Passwords don't match");
            }
        } else {
            println!("Wrong password");
        }
    } else if let Some(employee) = employees_list.iter().find(|x| x.login == _account_login) {
        if employee.password == _old_password {
            let new_password = get_password(Some("New password: "));
            let new_password_confirm = get_password(Some("Confirm new password: "));

            if new_password == new_password_confirm {
                update_password(
                    connection,
                    &employee.id,
                    &new_password,
                    false,
                    login,
                    str_date
                  );

                println!("Password updated successfully");
            } else {
                println!("Passwords don't match");
            }
        } else {
            println!("Wrong password");
        }
    } else {
        println!("User not found");
    }

}