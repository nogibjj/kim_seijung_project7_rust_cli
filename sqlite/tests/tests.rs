#[cfg(test)]
mod tests {
    // Import specific functions from mylib.rs
    use rusqlite::{params, Connection};
    use sqlite::{create_table, load_data, read, delete, update};
    use std::fs;

    // Helper function to set up an in-memory SQLite database for testing
    fn setup_test_db() -> rusqlite::Result<Connection> {
        let conn = Connection::open_in_memory()?;  // In-memory database for testing
        Ok(conn)
    }

    #[test]
    fn test_create_table() -> Result<(), Box<dyn std::error::Error>> {
        let conn = setup_test_db()?;
        let result = create_table(&conn, "employee");
        assert!(result.is_ok(), "Failed to create table");
        Ok(())
    }

    #[test]
    fn test_load_data() -> Result<(), Box<dyn std::error::Error>> {
        let conn = setup_test_db()?;
        create_table(&conn, "employee")?;

        // In-memory CSV data (usually this would come from a file)
        let data = "user_id,age,salary,years_of_experience\n\
                    1,28,50000,5\n\
                    2,34,65000,8\n\
                    3,25,45000,3\n\
                    4,41,80000,15\n\
                    5,30,55000,7";
        let file_path = "test_data.csv";
        fs::write(file_path, data).unwrap(); // Write data to a file

        let result = load_data(&conn, "employee", file_path);
        assert!(result.is_ok(), "Failed to load data into table");

        // Clean up the file after use
        fs::remove_file(file_path).unwrap();
        Ok(())
    }

    #[test]
    fn test_read() -> Result<(), Box<dyn std::error::Error>> {
        let conn = setup_test_db()?;
        create_table(&conn, "employee")?;

        // Insert data into the table
        conn.execute("INSERT INTO employee (user_id, age, salary, years_of_experience) VALUES (?1, ?2, ?3, ?4)", 
            params![1, 28, 50000, 5])?;

        // Run the read query
        let result = read(&conn, "SELECT * FROM employee WHERE user_id = 1");
        assert!(result.is_ok(), "Query execution failed");
        Ok(())
    }

    #[test]
    fn test_update_salary() -> Result<(), Box<dyn std::error::Error>> {
        let conn = setup_test_db()?;
        create_table(&conn, "employee")?;

        // Insert a test record
        conn.execute("INSERT INTO employee (user_id, age, salary, years_of_experience) VALUES (?1, ?2, ?3, ?4)",
            params![1, 28, 50000, 5])?;

        // Update the salary of the employee with user_id 1
        let result = update(&conn, "employee", 1, 28, 60000, 5);
        assert!(result.is_ok(), "Failed to update record");
        Ok(())
    }

    #[test]
    fn test_delete() -> Result<(), Box<dyn std::error::Error>> {
        let conn = setup_test_db()?;
        create_table(&conn, "employee")?;

        // Insert a test record
        conn.execute("INSERT INTO employee (user_id, age, salary, years_of_experience) VALUES (?1, ?2, ?3, ?4)",
            params![1, 28, 50000, 5])?;

        // Delete the record with user_id = 1
        let result = delete(&conn, "employee", 1);
        assert!(result.is_ok(), "Failed to delete record");

        // Verify that the record has been deleted
        let query = "SELECT age FROM employee WHERE user_id = 1";
        let mut stmt = conn.prepare(query)?;
        let mut rows = stmt.query([])?;
        assert!(rows.next()?.is_none(), "Record was not deleted");
        Ok(())
    }
}
