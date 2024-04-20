#[path ="lib.rs"]
mod lib;

use dialoguer::{Select, theme::ColorfulTheme};
use lib::select_dbms;

fn main() {
    println!("Welcome to Backup Restore DB CLI!");

    let selected_dbms = select_dbms();

    // check dump tool is available in system using std::process::Command
    let output = std::process::Command::new("which")
        .arg(&selected_dbms.dump_tool)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        println!("{} is available in system", selected_dbms.dump_tool);
    } else {
        println!("{} is not available in system", selected_dbms.dump_tool);
        println!("Please install {} and try again", selected_dbms.dump_tool);
    }

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
        },
        1 => {
            println!("You selected Restore");
            // call restore function
        },
        _ => println!("Invalid selection"),
    }
    
    println!("Goodbye!");
}

