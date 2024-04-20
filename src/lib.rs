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
