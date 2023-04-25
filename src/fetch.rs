use self::models::*;
use diesel::prelude::*;
use r_bilio::*;

pub fn fetch_data() {
    use self::schema::books::dsl::*;
    use self::schema::users::dsl::*;
    use self::schema::employees::dsl::*;

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

    // Display books
    println!("there is currently {} books", book_list.len());
    for book in book_list {
        println!("{}", book.name);
    }

    // Display users
    println!("there is currently {} users", user_list.len());
    for user in user_list {
        println!("{}", user.name);
    }

    // Dispaly employees
    println!("there is currently {} employees", employee_list.len());
    for employee in employee_list {
        println!("{}", employee.name);
    }
} 