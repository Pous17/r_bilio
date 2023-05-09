pub fn help() {
    let multiline_string = "
        Here are the commands you can use:

            - add -> params -> 
                - '-book' -> add a book
                - '-user' -> add a user
                - '-empl' -> add a employee

            - borrow -> borrow a book
            - borrow -> params -> 
                - '-list' -> list of currently borrowed books
                - '-list-id' -> list of currently borrowed books with the borrow id
                - '-list-date -> list of the currently borrowed books with the date they were borrowed
                - '-list-id-date -> list with both the date and id

            - exit -> quit the bilio manager
            - help -> show help
            - status -> get the current status of the bilio
            - status -> params ->
                - '-id' -> current status with id
                
            - populate -> populates the database with desired number of items
    ";
    println!("{}", multiline_string);
}
