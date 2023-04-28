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

            - exit -> quit the bilio manager
            - help -> show help
            - status -> get the current status of the bilio
            - status -> params ->
                - '-id' -> current status with id
    ";
    println!("{}", multiline_string);
}
