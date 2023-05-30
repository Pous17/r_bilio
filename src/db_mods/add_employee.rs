use r_bilio::{connection, create_employee};

use crate::{db_mods::fetch_db::fetch_employees, utils::{get_name, name_check, get_password}};


pub fn add_employee(login: &str, str_date: &str) {
    loop {
        println!("r_bilio > add employee >");
        let (employee_firstname, employee_lastname) = get_name("Name of the employee: ");

        if name_check(&employee_firstname, &employee_lastname, false) {
            let connection = &mut connection();
            let employees_list = fetch_employees();

            if let Some(employee) = employees_list.iter()
            .find(|x| x.firstname == employee_firstname && x.lastname == employee_lastname) {
                print!("{} {} already exists.", employee.firstname, employee.lastname);
                return;
            }

            let hash_pass = get_password(None);

            let employee = create_employee(
                connection,
                &employee_firstname,
                &employee_lastname,
                &hash_pass,
                login,
                str_date,
            );

            println!("New employee {} {} created \nwith id {} and login {}\n", 
            employee.firstname, employee.lastname, employee.id, employee.login);
            return;
        }
    }
}
