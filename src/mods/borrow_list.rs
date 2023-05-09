use super::super::db_mods::fetch_db::*;

pub fn list(param: &str) {
    let lists = fetch();
    let borrow_list = lists.3;

    println!("there is currently {} books", borrow_list.len());
    for borrow in borrow_list {
        if param == "id" {
            println!("Borrower: {} | Book: {} | Borrow id: {}", borrow.user_id, borrow.book_id, borrow.id);
        } else if param == "date" {
            println!("Borrower: {} | Book: {} | Borrow date: {}", borrow.user_id, borrow.book_id, borrow.borrow_date);
        } else if param == "id-date" {
            println!("Borrower: {} | Book: {} | Borrow date: {} | Borrow id: {}", borrow.user_id, borrow.book_id, borrow.borrow_date ,borrow.id)
        } else {
            println!("Borrower: {} | Book: {}", borrow.user_id, borrow.book_id);
        }
    }
}