use std::io::{stdin, stdout, Write};
use r_bilio::*;

pub fn add_employee() {
    let connection = &mut connection();

    loop {
        print!("r_bilio > add employee > \n");
        stdout().flush().unwrap();

        print!("Name of the employee: ");
        stdout().flush().unwrap();
        let mut employee_name = String::new();
        stdin().read_line(&mut employee_name).unwrap();

        if employee_name.trim() != "" {

            let employee = create_employee(connection, employee_name.trim_end());
            println!("New employee {} created with id {}", employee.name, employee.id);

            return
        } else {
            println!("Please enter valid data (empty names are not allowed)\n");
        }

    }
}