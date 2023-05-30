use crate::db_mods::fetch_db::*;


pub fn status(param: &str) {
    // Fetching data
    let all_lists = fetch_all();

    let users_list = all_lists.0;
    let authors_list = all_lists.1;
    let employees_list = all_lists.2;
    let books_list = all_lists.3;
    let past_borrows_list = all_lists.5;

    // Display books
    if param.contains("book") || param.contains("all") {
        let show_id = param.contains("-id");
        let show_info = param.contains("-info");
    
        println!("\nThere are currently {} books", books_list.len());
        println!("--------------");
    
        for book in &books_list {
            if show_id {
                println!("{} | id: {}", book.name, book.id);
            } else if show_info {
                println!("{} | {} | Currently borrowed: {}", book.name, format!("{} {}", book.author_firstname, book.author_lastname), book.borrowed);
            } else {
                println!("{}", book.name);
            }
        }
    }
    // Display author
    if param.contains("all") {

        println!("\nThere are currently {} authors", authors_list.len());
        println!("--------------");

        for author in &authors_list {
            let string = format!("{} {}", author.firstname, author.lastname);
            println!("{}", string.trim());
        }
    }

    // Display users
    if param.contains("user") || param.contains("all") {
        let show_id = param.contains("-id");
        let show_info = param.contains("-info");

        println!("\nThere are currently {} users", users_list.len());
        println!("--------------");

        for user in &users_list {
            if show_id {
                println!("{} {} | id: {}", user.firstname, user.lastname, user.id);
            } else if show_info {
                println!("{} {} | Is a member: {} | Score: {}", user.firstname,  user.lastname, user.member, user.score);
            } else {
                println!("{} {}", user.firstname, user.lastname);
            }
        }
    }

    // Display employees
    if param.contains("empl") || param.contains("all") {
        let show_id = param.contains("-id");

        println!("\nThere are currently {} employees", employees_list.len());
        println!("--------------");

        for employee in &employees_list {
            if show_id {
                println!("{} {} | id: {}", employee.firstname, employee.lastname, employee.id);
            } else {
                println!("{} {}", employee.firstname, employee.lastname);
            }
        }
    }

    // Display borrow logs
    if param.contains("logs") {
        println!("--------------");
        for log in &past_borrows_list {
            println!(
                "User id: {} | Book id: {} | Returned damaged: {} | Borrow date: {} | Return date: {}",
                log.user_id, log.book_id, log.damaged, log.borrow_date, log.return_date
            );
        }
    }
}