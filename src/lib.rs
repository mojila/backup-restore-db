use std::process::exit;

use dialoguer::{theme::ColorfulTheme, Select};

pub struct DatabaseManagementSystem {
    pub name: String,
    pub dump_tool: String,
}

impl DatabaseManagementSystem {
    pub fn clone(&self) -> DatabaseManagementSystem {
        DatabaseManagementSystem {
            name: self.name.clone(),
            dump_tool: self.dump_tool.clone(),
        }
    }

    pub fn dump(&self) {
        println!("Dumping database using {}", self.dump_tool);

        let dump_tool = self.dump_tool.clone();

        // check dump tool is available in system using std::process::Command
        let output = std::process::Command::new("which")
            .arg(&dump_tool)
            .output()
            .expect("failed to execute process");

        if output.status.success() {
            println!("{} is available in system", self.dump_tool);
        } else {
            println!("{} is not available in system", self.dump_tool);
            println!("Please install {} and try again", self.dump_tool);
            exit(1)
        }

        // call dump tool
        match dump_tool.as_str() {
            "pg_dump" => {
                println!("Dumping PostgreSQL database");

                // set environment variable PGPASSWORD
                std::env::set_var("PGPASSWORD", "your_password");

                // call pg_dump
                let log = std::process::Command::new("/usr/bin/pg_dump")
                    .args(&[
                        "--file",
                        "/home/sandbox/Documents/backup-1.sql",
                        "--host",
                        "127.0.0.1",
                        "--port",
                        "5432",
                        "--username",
                        "your_username",
                        "--verbose",
                        "--format=c",
                        "--blobs",
                        "your_database_name",
                    ])
                    .spawn()
                    .expect("failed to execute pg_dump");

                log.wait_with_output().expect("failed to wait on child");
            },
            "mongodump" => {
                println!("Dumping MongoDB database");
                // call mongodump
            },
            _ => {
                println!("Dump tool not supported");
                exit(1)
            },
        }

        println!("Database dumped successfully");
    }

    pub fn restore(&self) {
        println!("Restoring database using {}", self.dump_tool);
    }
}

// make vec of DatabaseManagementSystem
pub fn get_dbms_options() -> Vec<DatabaseManagementSystem> {
    vec![
        DatabaseManagementSystem { name: String::from("PostgreSQL"), dump_tool: String::from("pg_dump") },
        DatabaseManagementSystem { name: String::from("MongoDB"), dump_tool: String::from("mongodump") },
    ]
}

pub fn select_dbms() -> DatabaseManagementSystem {
    // select using dialoguer with options from get_dbms_options()
    let binding: Vec<DatabaseManagementSystem> = get_dbms_options();
    let dbms_options = binding.iter().map(|dbms| dbms.name.as_str()).collect::<Vec<&str>>();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select Database Management System")
        .items(&dbms_options)
        .default(0)
        .interact()
        .unwrap();

    let selected_dbms: DatabaseManagementSystem = binding[selection].clone();

    println!("You selected: {}", selected_dbms.name);

    return selected_dbms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone_dbms() {
        let dbms = DatabaseManagementSystem {
            name: String::from("PostgreSQL"),
            dump_tool: String::from("pg_dump"),
        };

        let cloned_dbms = dbms.clone();

        assert_eq!(cloned_dbms.name, dbms.name);
        assert_eq!(cloned_dbms.dump_tool, dbms.dump_tool);
    }

    #[test]
    fn test_get_dbms_options() {
        let dbms_options = get_dbms_options();

        assert_eq!(dbms_options.len(), 2);

        let postgres_dbms = &dbms_options[0];
        assert_eq!(postgres_dbms.name, "PostgreSQL");
        assert_eq!(postgres_dbms.dump_tool, "pg_dump");

        let mongodb_dbms = &dbms_options[1];
        assert_eq!(mongodb_dbms.name, "MongoDB");
        assert_eq!(mongodb_dbms.dump_tool, "mongodump");
    }
}
