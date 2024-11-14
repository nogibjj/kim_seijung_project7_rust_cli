#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use std::fs;

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
    fn test_load_data() {
        let file_path = "test_data.csv";
        // Set up test data with unique user_id to avoid the constraint violation
        let data = "user_id,age,salary,years_of_experience\n\
                    6,28,50000,5\n\
                    7,34,65000,8\n\
                    8,25,45000,3\n\
                    9,41,80000,15\n\
                    10,30,55000,7";
        std::fs::write(file_path, data).unwrap();

        // First delete any existing records to prevent constraint violations
        let mut cmd = Command::cargo_bin("sqlite").unwrap();
        cmd.arg("delete")
            .arg("employee")
            .arg("1")
            .assert()
            .success(); // Ensure the command was successful

        // Now load the data
        let mut cmd = Command::cargo_bin("sqlite").unwrap();
        cmd.arg("load")
            .arg("employee")
            .arg(file_path)
            .assert()
            .success() // Ensure the command was successful
            .stdout(predicates::str::contains(
                "Loading data into table 'employee' from",
            )) // Verify expected output
            .stderr(""); // No error output expected

        // Clean up the file after use
        std::fs::remove_file(file_path).unwrap();
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
