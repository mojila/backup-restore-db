pub struct DatabaseManagementSystem {
    pub name: String,
    pub dump_tool: String,
}

// make vec of DatabaseManagementSystem
pub fn get_dbms_options() -> Vec<DatabaseManagementSystem> {
    vec![
        DatabaseManagementSystem { name: String::from("PostgreSQL"), dump_tool: String::from("pg_dump") },
        DatabaseManagementSystem { name: String::from("MongoDB"), dump_tool: String::from("mongodump") },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

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