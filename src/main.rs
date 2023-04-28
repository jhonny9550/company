// Import project mods
mod company;
mod interface;

// Import crates
use crate::company::*;
use crate::interface as TextInterface;

fn print_department_employers(company: &Company, dep: &Department) -> Result<(), String> {
    match company.get(&dep) {
        Some(employers) => {
            println!("{:?}", employers);
            Ok(())
        }
        None => Err(format!("Department {:?} not found", &dep)),
    }
}

fn process_user_input(company: &mut Company, input: &str) -> Result<(), String> {
    // Process user input
    if input.starts_with("add") || input.starts_with("Add") {
        let mut commands = input.split_whitespace();
        commands.next();
        let employee_name = commands.next().unwrap();
        commands.next();
        match Department::from_str(commands.next().unwrap()) {
            Some(dep) => {
                company.add_employer(dep, &employee_name);
                Ok(())
            }
            None => Err(String::from("Unknown department")),
        }
    } else {
        let command: char = input.chars().next().unwrap();
        match command {
            '1' => {
                println!("{:?}", company);
                Ok(())
            }
            '2' => print_department_employers(&company, &Department::Design),
            '3' => print_department_employers(&company, &Department::Engineering),
            '4' => print_department_employers(&company, &Department::Marketing),
            '5' => print_department_employers(&company, &Department::Sales),
            'q' => std::process::exit(0),
            _ => Ok(()),
        }
    }
}

fn main() {
    // Create company
    let mut company = Company::new();

    // Draw the interface
    TextInterface::draw_options();
    loop {
        // Read user input
        match TextInterface::read_user_input() {
            Ok(input) => {
                if let Err(e) = process_user_input(&mut company, &input) {
                    eprintln!("{}", e);
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
