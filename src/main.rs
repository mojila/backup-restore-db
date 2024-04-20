#[path ="lib.rs"]
mod lib;

use dialoguer::{Select, theme::ColorfulTheme};

fn main() {
    println!("Welcome to Backup Restore DB CLI!");

    // select using dialoguer with options from get_dbms_options()
    let binding = lib::get_dbms_options();
    let dbms_options = binding.iter().map(|dbms| dbms.name.as_str()).collect::<Vec<&str>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Database Management System")
        .items(&dbms_options)
        .default(0)
        .interact()
        .unwrap();

    let selected_dbms = &binding[selection];

    println!("You selected: {}", selected_dbms.name);

    // check dump tool is available in system using std::process::Command
    let output = std::process::Command::new("which")
        .arg(&selected_dbms.dump_tool)
        .output()
        .expect("failed to execute process");

    if output.status.success() {
        println!("{} is available in system", selected_dbms.dump_tool);
    } else {
        println!("{} is not available in system", selected_dbms.dump_tool);
    }
    
    println!("Goodbye!");
}

