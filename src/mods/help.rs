pub fn help() {
    let multiline_string = "
        Here are the commands you can use:

            - add -> params -> 
                - '-book' -> add a book
                - '-user' -> add a user
                - '-empl' -> add a employee
                
            - exit -> quit the bilio manager
            - help -> show help
            - status -> get the current status of the bilio
    ";
    println!("{}", multiline_string);
}
