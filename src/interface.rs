use crate::company::Department;
use std::io;

// Draw the options available for the user
pub fn draw_options() {
    println!("Add an employer typing 'Add [name] to [department] and pressing Enter.\n");
    println!(
        "Available departments: {:?}.\n",
        [
            Department::Design,
            Department::Engineering,
            Department::Marketing,
            Department::Sales
        ]
    );
    println!("Or choose one of the following options:\n");
    println!("(1) List all people in the company");
    println!("(2) List all people in the Design department");
    println!("(3) List all people in the Engineering department");
    println!("(4) List all people in the Marketing department");
    println!("(5) List all people in the Sales department");
    println!("(q) Exit program");
}

const VALID_OPTIONS: &[char] = &['q', '1', '2', '3', '4', '5'];

// Validates if user input has the required format
fn validate_user_input(input: &str) -> bool {
    if input.starts_with("Add") || input.starts_with("add") {
        let commands = input.split(' ');
        let command_size = commands.clone().count();
        if command_size != 4 {
            false
        } else {
            let mut iter = commands.into_iter();
            iter.next();
            iter.next();
            if iter.next() != Some("to") {
                false
            } else {
                Department::is_valid(iter.next())
            }
        }
    } else {
        input.len() == 1 && input.starts_with(VALID_OPTIONS)
    }
}

// Ask user input
pub fn read_user_input() -> Result<String, String> {
    let mut user_input = String::from("");

    print!("\n");

    io::stdin().read_line(&mut user_input).expect("Wrong input");

    // Remove break line \n character from input
    user_input.pop();

    // If user input is wrong, ask again
    if !validate_user_input(&user_input) {
        eprintln!("Wrong input! Try again.");
        read_user_input()
    } else {
        Ok(user_input)
    }
}
