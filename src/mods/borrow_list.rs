use super::super::db_mods::fetch_db::*;

pub fn list(param: &str) {
    let lists = fetch();
    let borrow_list = lists.3;

    println!("there is currently {} borrows", borrow_list.len());
    println!("--------------");
    for borrow in borrow_list {
        match param {
            "id" => println!("Borrower: {} | Book: {} | Borrow id: {}", borrow.user_id, borrow.book_id, borrow.id),
            "date" => println!("Borrower: {} | Book: {} | Borrow date: {}", borrow.user_id, borrow.book_id, borrow.borrow_date),
            "id-date" => println!("Borrower: {} | Book: {} | Borrow date: {} | Borrow id: {}", borrow.user_id, borrow.book_id, borrow.borrow_date ,borrow.id),
            _ => println!("Borrower: {} | Book: {}", borrow.user_id, borrow.book_id)
        }
    }
}