use std::io::{stdin, stdout, Write};
use r_bilio::*;

pub fn add_employee() {
    let connection = &mut connection();

    loop {
        println!("r_bilio > add employee >");

        let employee_name = input_string("Name of the employee: ");

        if !employee_name.is_empty() {
            let employee = create_employee(connection, &employee_name);
            println!("New employee {} created with id {}\n", employee.name, employee.id);
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