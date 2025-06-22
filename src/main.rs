use std::io;

mod flmg;
use flmg::{file_explorer, file_editor};


fn main() {

    let mut input: String = String::new();

    println!("\nWelcome to Longinus Manager!\n\nType:\n1 => to enter into the file explorer\n2 => to enter into the file editor\nq => to quit\n");

    loop {

        input.clear();
        
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                file_explorer::call_file_explorer();
                break;
            }
            "2" => {
                file_editor::call_file_editor();
                break;
            }
            "q" => {
                println!("\nGoodbye!");
                break;
            }
            _ => {
                println!("\nInvalid option. Please try again.");
                continue;
            }
        }
    }

}