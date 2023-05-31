use crate::db_mods::fetch_db::*;


pub fn status(param: &str) {
    // Fetching data
    let all_lists = fetch_all();

    let users_list = all_lists.0;
    let authors_list = all_lists.1;
    let employees_list = all_lists.2;
    let books_list = all_lists.3;
    let borrows_list = all_lists.4;
    let past_borrows_list = all_lists.5;
    
    let show_id = param.contains("-id");
    let show_info = param.contains("-info");

    // Display books
    if param.contains("book") || param.contains("all") {
        println!("\nThere are currently {} books", books_list.len());
        println!("--------------");

        if show_id {
            println!("Name | Id");
        } else if show_info {
            println!("Name | Id | Author | Borrowed");
        } else {
            println!("Name");
        }
    
        for book in &books_list {
            if show_id {
                println!("{} | {}", book.name, book.id);
            } else if show_info {
                println!("{} | {} | {} | {}", book.name, book.id, format!("{} {}", book.author_firstname, book.author_lastname).trim(), book.borrowed);
            } else {
                println!("{}", book.name);
            }
        }
    }
    // Display author
    if param.contains("author") || param.contains("all") {
        println!("\nThere are currently {} authors", authors_list.len());
        println!("--------------");

        if show_id || show_info {
            println!("Name | Id");
        } else {
            println!("Name");
        }

        for author in &authors_list {
            let string = format!("{} {}", author.firstname, author.lastname);
            if show_id || show_info {
                println!("{} | {}", string.trim(), author.id);
            } else {
                println!("{}", string.trim());
            }
        }
    }

    // Display users
    if param.contains("user") || param.contains("all") {
        println!("\nThere are currently {} users", users_list.len());
        println!("--------------");

        if show_id {
            println!("Name | Id");
        } else if show_info {
            println!("Name | Id | Login | Membership | Score");
        } else {
            println!("Name");
        }

        for user in &users_list {
            if show_id {
                println!("{} {} | {}", user.firstname, user.lastname, user.id);
            } else if show_info {
                println!("{} {} | {} | {} | {} | {}", user.firstname,  user.lastname, user.id, user.login, user.member, user.score);
            } else {
                println!("{} {}", user.firstname, user.lastname);
            }
        }
    }

    // Display employees
    if param.contains("empl") || param.contains("all") {
        println!("\nThere are currently {} employees", employees_list.len());
        println!("--------------");

        if show_id {
            println!("Name | Id");
        } else if show_info {
            println!("Name | Id | Login");
        } else {
            println!("Name");
        }

        for employee in &employees_list {
            if show_id {
                println!("{} {} | {}", employee.firstname, employee.lastname, employee.id);
            } else if show_info {
                println!("{} {} | {} | {}", employee.firstname, employee.lastname, employee.id, employee.login);
            } else {
                println!("{} {}", employee.firstname, employee.lastname);
            }
        }
    }

    // Display borrows
    if param.contains("borrow") || param.contains("all") {
        println!("\nThere are currently {} borrows", borrows_list.len());
        println!("--------------");

        if show_id {
            println!("User id | Book id | Borrow id");
        } else if show_info {
            println!("User id | Book id | Borrow date | Limit date | Borrow id");
        } else {
            println!("User id | Book id");
        }

        for borrow in &borrows_list {
            if show_id {
                println!("{} | {} | {}", borrow.user_id, borrow.book_id, borrow.id);
            } else if show_info {
                println!("{} | {} | {} | {} | {}", borrow.user_id, borrow.book_id, borrow.borrow_date, borrow.limit_date, borrow.id);
            } else {
                println!("{} | {}", borrow.user_id, borrow.book_id);
            }
        }
    }

    // Display past borrows
    if param.contains("pastborrow") || param.contains("all") {
        println!("\nThere are currently {} past borrows", past_borrows_list.len());
        println!("--------------");

        if show_id {
            println!("User id | Book id | Borrow id");
        } else if show_info {
            println!("User id | Book id | Damaged | Late | Return Date | Borrow id");
        } else {
            println!("User id | Book id");
        }

        for borrow in &past_borrows_list {
            if show_id {
                println!("{} | {} | {}", borrow.user_id, borrow.book_id, borrow.id);
            } else if show_info {
                println!("{} | {} | {} | {} | {} | {}", borrow.user_id, borrow.book_id, borrow.damaged, borrow.late, borrow.return_date, borrow.id);
            } else {
                println!("{} | {}", borrow.user_id, borrow.book_id);
            }
        }
    }
}