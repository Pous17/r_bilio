use self::models::*;
use diesel::prelude::*;
use r_bilio::*;

pub fn fetch() -> (Vec<Books>, Vec<Users>, Vec<Employees>, Vec<Borrows>, Vec<PastBorrows>) {
    use self::schema::books::dsl::*;
    use self::schema::books::dsl::id as book_id;
    use self::schema::users::dsl::*;
    use self::schema::users::dsl::id as user_id;
    use self::schema::employees::dsl::*;
    use self::schema::employees::dsl::id as empl_id;
    use self::schema::borrows::dsl::*;
    use self::schema::borrows::dsl::id as borrow_id;
    use self::schema::past_borrows::dsl::*;
    use self::schema::past_borrows::dsl::id as past_borrows_id;

    let connection = &mut connection();

    // Fetching data
    let book_list = books
        .order(book_id)
        .load::<Books>(connection)
        .expect("Error loading books");

    let user_list = users
        .order(user_id)
        .load::<Users>(connection)
        .expect("Error loading users");

    let employee_list = employees
        .order(empl_id)
        .load::<Employees>(connection)
        .expect("Error loading employees");

    let borrow_list = borrows
        .order(borrow_id)
        .load::<Borrows>(connection)
        .expect("Error loading current borrows");

    let past_borrows_list = past_borrows
        .order(past_borrows_id)
        .load::<PastBorrows>(connection)
        .expect("Error loading past borrows");

    (book_list, user_list, employee_list, borrow_list, past_borrows_list)
}