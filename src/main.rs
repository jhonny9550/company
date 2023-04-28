// Import project mods
mod company;
mod interface;

// Import crates
use crate::company::*;
use crate::interface as TextInterface;

fn main() {
    // Create company
    let mut company = Company::new();

    // Draw the interface
    TextInterface::draw_options();

    // Read user input
    let user_input = TextInterface::read_user_input();

    // Process user input
    println!("User Input: {user_input}");
}
