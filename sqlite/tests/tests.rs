#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use rusqlite::{Connection, Result};
    use std::fs;

    // Helper function to set up an in-memory SQLite database for testing
    fn setup_test_db() -> rusqlite::Result<Connection> {
        let conn = Connection::open_in_memory()?; // In-memory database for testing
        Ok(conn)
    }

    #[test]
    fn test_create_table() {
        let mut cmd = Command::cargo_bin("sqlite").unwrap();
        cmd.arg("create")
            .arg("employee")
            .assert()
            .success() // Ensure the command was successful
            .stdout(predicates::str::contains("Creating Table 'employee'")) // Verify the expected output
            .stderr(""); // No error output expected
    }

    #[test]
    fn test_read_data() {
        let mut cmd = Command::cargo_bin("sqlite").unwrap();
        cmd.arg("read")
            .arg("SELECT * FROM employee WHERE user_id = 1")
            .assert()
            .success() // Ensure the command was successful
            .stdout(predicates::str::contains(
                "Executing Query: SELECT * FROM employee WHERE user_id = 1",
            )) // Adjusted expected output
            .stderr(""); // No error output expected
    }

    #[test]
    fn test_update_data() {
        let mut cmd = Command::cargo_bin("sqlite").unwrap();
        cmd.arg("update")
            .arg("employee")
            .arg("1") // user_id
            .arg("35") // age
            .arg("60000") // salary
            .arg("8") // years_of_experience
            .assert()
            .success() // Ensure the command was successful
            .stdout(predicates::str::contains("Updating Record with user_id 1")) // Expected output
            .stderr(""); // No error output expected
    }

    #[test]
    fn test_delete_data() {
        let mut cmd = Command::cargo_bin("sqlite").unwrap();
        cmd.arg("delete")
            .arg("employee")
            .arg("6") // user_id to delete
            .assert()
            .success() // Ensure the command was successful
            .stdout(predicates::str::contains(
                "Deleting Record with user_id 6 from table employee",
            )) // Expected output
            .stderr(""); // No error output expected
    }
}
