# Employee Management System

This Rust program is a text interface that allows a user to add employee names to departments in a company. The program uses a hash map and vectors to store the data. The user can also retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

## Requirements

- Rust programming language (version 1.53 or newer)

## Usage

To run the program, open a terminal window, navigate to the project directory, and run the following command:

```
cargo run
```

The program will display a prompt:

```
Add an employer typing `Add <name> to <department>` and pressing Enter.

Available departments: Design, Engineering, Marketing and Sales.

Or choose one of the following options:

(1) List all people in the company
(2) List all people in the Design department
(3) List all people in the Engineering department
(4) List all people in the Marketing department
(5) List all people in the Sales department
(q) Exit program

>
```

The user can then enter commands to add employees to departments or retrieve lists of employees.

## License

This program is licensed under the MIT License. See the LICENSE file for details.
