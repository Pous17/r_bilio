use std::io::{stdin, stdout, Write};
use r_bilio::*;

use super::fetch_db::fetch;

pub fn add_book() {
    let connection = &mut connection();

    let lists = fetch();
    let author_list = lists.5;

    loop {
        println!("r_bilio > add book >");

        let book_name = input_string("Name of the book: ");
        let author_name = input_string("Author of the book: ");
        let mut author_firstname = author_name.split(" ").next().unwrap();
        let mut author_lastname = author_name.split(" ").last().unwrap();
        let mut author_id: &i32 = &0;
        let temp_firstname: String;
        let temp_lastname: String;
        let temp_id: i32;

        if author_firstname == author_lastname {
            author_firstname = "";
        }

        if !book_name.is_empty() && !author_name.is_empty() {
            if let Some(author) = author_list.iter().find(|x| x.lastname == author_lastname) {
                author_firstname = &author.firstname;
                author_lastname = &author.lastname;
                author_id = &author.id;
            } 

            if author_id == &0 {
                let author = create_author(connection, author_firstname, author_lastname);
                
                temp_firstname = author.firstname.clone();
                author_firstname = &temp_firstname;
                
                temp_lastname = author.lastname.clone();
                author_lastname = &temp_lastname;

                temp_id = author.id.clone();
                author_id = &temp_id;

            }
            let book = create_book(
                connection, 
                &book_name, 
                &author_id,
                &author_firstname,
                &author_lastname,
                &false
            );

           println!("New book {} added with id {}\n", book_name, book.id);
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