use crate::db_mods::fetch_db::fetch_borrows;


pub fn list(param: &str) {
    let borrows_list = fetch_borrows();

    println!("there is currently {} borrows", borrows_list.len());
    println!("--------------");
    for borrow in borrows_list {
        match param {
            "id" => println!("Borrower: {} | Book: {} | Borrow id: {}", borrow.user_id, borrow.book_id, borrow.id),
            "date" => println!("Borrower: {} | Book: {} | Borrow date: {}", borrow.user_id, borrow.book_id, borrow.borrow_date),
            "id-date" => println!("Borrower: {} | Book: {} | Borrow date: {} | Borrow id: {}", borrow.user_id, borrow.book_id, borrow.borrow_date ,borrow.id),
            _ => println!("Borrower: {} | Book: {}", borrow.user_id, borrow.book_id)
        }
    }
}