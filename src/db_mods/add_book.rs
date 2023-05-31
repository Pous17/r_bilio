use r_bilio::*;

use crate::utils::{get_name, name_check, input_string, input_i32};

use super::fetch_db::fetch_authors;


pub fn add_book(login: &str, str_date: &str) {
    let connection = &mut connection();
    let authors_list = fetch_authors();

    let mut author_firstname;
    let mut author_lastname;
    let mut author_id: i32 = -1;

    println!("r_bilio > add book >");

    let book_name = input_string("Name of the new book: ");
    let choice_by_id = input_string("Select author by id ? (y/N) ").to_lowercase();
    let by_id = match choice_by_id.as_str() {
        "y" => true,
        "n" => false,
         _ => false,
    };

    if by_id {
        let _input_id = input_i32("Enter author id: ").clone();

        if _input_id < 0 {
            println!("Invalid Id number");
            return
        }

        if let Some(author) = authors_list.iter().find(|x| x.id == _input_id) {
            let book = create_book(
                connection, 
                &book_name,
                login,
                str_date,
                &author.id,
                &author.firstname,
                &author.lastname,
            );

            let author_name = format!("{} {}", author.firstname, author.lastname);
            println!("New book \"{}\" by {} added with id {}\n", book_name, author_name, book.id);
            return
        } else {
            println!("Author with id {} not found", author_id);
            return
        }
    } else {
        (author_firstname, author_lastname) = get_name("Name of the author: ");

        if name_check(&author_firstname, &author_lastname, true)  {    
            if let Some(author) = authors_list.iter().find(|x| x.lastname == author_lastname) {
                author_firstname = author.firstname.clone();
                author_lastname = author.lastname.clone();
                author_id = author.id;
            } 
    
            if author_id == -1 {
                let author_name = format!("{} {}", author_firstname, author_lastname);
                    
                let _choice_add_author = input_string(format!("Did not find {} in the database, create it ? (Y/n) ", author_name.trim()).as_str()).to_lowercase();
                let add_author = match _choice_add_author.as_str() {
                    "y" => true,
                    "n" => false,
                    _ => true,
                };

                if add_author {
                    let author = create_author(
                        connection,
                        &author_firstname,
                        &author_lastname,
                        login,
                         str_date,
                    );

                    author_id = author.id;
                    author_firstname = author.firstname.clone();
                    author_lastname = author.lastname.clone();

                    println!("New author {} created with id {}\n", author_name.trim(), author.id.clone());
                } else {
                    return
                }
            }
                
            let book = create_book(
                connection, 
                &book_name,
                login,
                str_date,
                &author_id,
                &author_firstname,
                &author_lastname,
            );
    
            let author_name = format!("{} {}", author_firstname, author_lastname);
            println!("New book \"{}\" by {} added with id {}\n", book_name, author_name, book.id);
            return
        } else {
            return
        }
    }
}
