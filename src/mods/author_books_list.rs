use r_bilio::models::Book;

use crate::{db_mods::fetch_db::{fetch_books_author_name, fetch_books_author_id}, utils::input_string};

pub fn author_books() {

    let books_list: Vec<Book>;

    let choice = input_string("Do you want to search by id (y) or by name (n)? (y/N)");
    let by_id: bool;
    let search_input: String;

    match choice.as_str() {
        "y" => {
            by_id = true;
            search_input = input_string("Enter author id: ");
        }
        "n" => {
            by_id = false;
            search_input = input_string("Enter author name: ");
        }
        _ => {
            println!("Invalid choice, defaulting to name");
            search_input = input_string("Enter author name: ");
            by_id = false;
        }
    }
    
    // Fetching books data
    if by_id {
        let author_id = search_input.parse::<i32>().unwrap_or(-1);
        books_list = fetch_books_author_id(&author_id);
    } else {
        let author_lastname = search_input.split_whitespace().last().unwrap_or("");
        books_list = fetch_books_author_name(author_lastname);
    }

    println!("\nThis author currently has {} books registered", books_list.len());
    println!("--------------");

    for book in &books_list {
        println!("Id: {} | Name: {}", book.id, book.name);
    }

}