use super::super::db_mods::fetch_db::*;

pub fn status(param: &str) {
    let lists = fetch();

    let book_list = lists.0;
    let user_list = lists.1;
    let employee_list = lists.2;

    // Display books
    println!("\nthere is currently {} books", book_list.len());
    println!("--------------");
    for book in book_list {
        if param == "id" {
            println!("{} | id: {}", book.name, book.id);
        } else {
            println!("{}", book.name);
        }
    }

    // Display users
    println!("\nthere is currently {} users", user_list.len());
    println!("--------------");
    for user in user_list {
        if param == "id" {
            println!("{} | id: {}", user.name, user.id);
        } else {
            println!("{}", user.name);
        }
    }

    // Dispaly employees
    println!("\nthere is currently {} employees", employee_list.len());
    println!("--------------");
    for employee in employee_list {
        if param == "id" {
            println!("{} | id: {}", employee.name, employee.id);
        } else {
            println!("{}", employee.name);
        }
    }
} 