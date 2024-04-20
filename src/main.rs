#[path ="lib.rs"]
mod lib;

use dialoguer::{Select, theme::ColorfulTheme};
use lib::select_dbms;

fn main() {
    println!("Welcome to Backup Restore DB CLI!");

    let selected_dbms = select_dbms();

    let action_list = vec!["Backup", "Restore"];

    let action = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Action")
        .items(&action_list)
        .default(0)
        .interact()
        .unwrap();

    match action {
        0 => {
            println!("You selected Backup");
            // call backup function
            selected_dbms.dump()
        },
        1 => {
            println!("You selected Restore");
            // call restore function
            selected_dbms.restore()
        },
        _ => println!("Invalid selection"),
    }
    
    println!("Goodbye!");
}

