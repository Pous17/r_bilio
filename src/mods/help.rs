pub fn help(param: &str) {
    let admin_multiline_string = "
        Here are the commands you can use:

            - add -> params -> 
                - '-book' -> add a book
                - '-user' -> add a user
                - '-empl' -> add a employee

            - author -> params -> 
                - '-book' -> list of books by author

            - borrow -> borrow a book
            - borrow -> params -> 
                - '-list' -> list of currently borrowed books
                - '-list-id' -> list of currently borrowed books with the borrow id
                - '-list-date' -> list of the currently borrowed books with the date they were borrowed
                - '-list-id-date' -> list with both the date and id
                - '-list-user' -> borrow list for a specific user
            
            - return -> return a book

            - status -> get the current status of the bilio
            - status -> params ->
                - '-id' -> current general status with id
                - '-info -> current general status with all information
                - '-author' -> list of authors
                - '-author-id' -> list of authors with id
                - '-book' -> books status
                - '-book-id' -> books status with id
                - '-book-info' -> books status with all information
                - '-user' -> users status
                - '-user-id' -> users status with id
                - '-user-info' -> users status with info
                - '-empl' -> employees status
                - '-empl-id' -> employees status with id
                - '-empl-info' -> employees status with info
                - '-borrow' -> borrows status
                - '-borrow-id' -> borrows status with id
                - '-borrow-info' -> borrows status with info
                - '-pastborrow' -> past borrows status
                - '-pastborrow-id' -> past borrows status with id
                - '-pastborrow-info' -> past borrows status with info
            
            - update -> params -> 
                - '-member' -> update membership
                - '-password' -> update password
                
            - populate -> populates the database with desired number of items
            - exit -> quit the bilio manager
            - help -> show help
    ";

    let user_multiline_string = "
        Here are the commands you can use: 
            - author -> params -> 
                - '-book' -> list of books by author

            - status -> params ->
                - '-book' -> books status
                - '-book-id' -> books status with id

            - 'update' -> params ->
                - '-password' -> update your password
            
            - exit -> quit the bilio manager
            - help -> show help
    ";

    if param == "employee" {
        println!("{}", admin_multiline_string);
    } else {
        println!("{}", user_multiline_string);
    }
}
