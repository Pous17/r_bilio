use r_bilio::*;


pub fn update_password(login: &str, str_date: &str) {
    let connection = &mut connection();
    let (users_list, employees_list) = fetch_users_employees();

    let old_password = get_password("Old password: ");

    if let Some(user) = users_list.iter().find(|x| x.login == login) {
        if user.password == old_password {
            let new_password = get_password("New password: ");
            let new_password_confirm = get_password("Confirm new password: ");

            if new_password == new_password_confirm {
                update_password(
                    connection,
                    &user.id,
                    &new_password,
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
    } else if let Some(employee) = employees_list.iter().find(|x| x.login == login) {
        if employee.password == old_password {
            let new_password = get_password("New password: ");
            let new_password_confirm = get_password("Confirm new password: ");

            if new_password == new_password_confirm {
                update_password(
                    connection,
                    &employee.id,
                    &new_password,
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