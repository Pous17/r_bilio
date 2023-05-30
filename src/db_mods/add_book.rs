use r_bilio::*;

use crate::utils::{get_name, name_check, input_string};

use super::fetch_db::fetch_authors;


pub fn add_book(login: &str, str_date: &str) {
    loop {
        println!("r_bilio > add book >");

        let book_name = input_string("Name of the book: ");
        let (mut author_firstname, mut author_lastname) = get_name("Name of the author: ");
        let mut author_id: &i32 = &0;

        if name_check(&author_firstname, &author_lastname, true)  {
            let connection = &mut connection();
            let authors_list = fetch_authors();

            if let Some(author) = authors_list.iter().find(|x| x.lastname == author_lastname) {
                author_firstname = author.firstname.clone();
                author_lastname = author.lastname.clone();
                author_id = &author.id;
            } 

            if author_id == &0 {
                let author = create_author(
                    connection,
                    &author_firstname,
                    &author_lastname,
                    login,
                    str_date,
                );
                
                let author_name = format!("{} {}", author.firstname, author.lastname);

                println!("New author {} created with id {}\n", author_name.trim(), author.id.clone());
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

           println!("New book {} added with id {}\n", book_name, book.id);
            return;
        }
    }
}
