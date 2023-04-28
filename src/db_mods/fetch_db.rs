use self::models::*;
use diesel::prelude::*;
use r_bilio::*;

pub fn fetch() -> (Vec<Books>, Vec<Users>, Vec<Employees>, Vec<Borrows>) {
    use self::schema::books::dsl::*;
    use self::schema::users::dsl::*;
    use self::schema::employees::dsl::*;
    use self::schema::borrows::dsl::*;

    let connection = &mut connection();

    // Fetching data
    let book_list = books
        .load::<Books>(connection)
        .expect("Error loading books");

    let user_list = users
        .load::<Users>(connection)
        .expect("Error loading users");

    let employee_list = employees
        .load::<Employees>(connection)
        .expect("Error loading employees");

    let borrow_list = borrows
        .load::<Borrows>(connection)
        .expect("Error loading current borrows");

    (book_list, user_list, employee_list, borrow_list)
}