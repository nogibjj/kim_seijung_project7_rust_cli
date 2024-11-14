use csv::ReaderBuilder;
use rusqlite::{Connection, Result};
use std::fs::File;

/// Create a table in the SQLite database
pub fn create_table(conn: &Connection, table: &str) -> Result<()> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            user_id INTEGER PRIMARY KEY,
            age INTEGER,
            salary INTEGER,
            years_of_experience INTEGER
        )",
        table
    );
    conn.execute(&create_query, [])?;
    println!("Table '{}' created successfully.", table);
    Ok(())
}

/// Read each row in the CSV and insert the data into the specified table.
pub fn load_data(
    conn: &Connection,
    table: &str,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new().from_reader(File::open(file_path)?);

    for record in rdr.records() {
        let record = record?;

        // Parse the fields from the CSV
        let user_id: i32 = record[0].parse()?;
        let age: i32 = record[1].parse()?;
        let salary: i32 = record[2].parse()?;
        let years_of_experience: i32 = record[3].parse()?;

        // Insert the data into the database
        conn.execute(
            &format!(
                "INSERT INTO {} (user_id, age, salary, years_of_experience)
                 VALUES (?1, ?2, ?3, ?4)",
                table
            ),
            [user_id, age, salary, years_of_experience], // no reference to array
        )?;
    }

    println!("Data loaded successfully into table '{}'.", table);
    Ok(())
}

/// Update user's age, salary, and years_of_experience based on their user_id.
pub fn update(
    conn: &Connection,
    table: &str,
    user_id: i32,
    age: i32,
    salary: i32,
    years_of_experience: i32,
) -> Result<()> {
    let update_query = format!(
        "UPDATE {} SET age = ?1, salary = ?2, years_of_experience = ?3 WHERE user_id = ?4",
        table
    );

    // Execute the update query, ensuring the types are correctly matched
    conn.execute(
        &update_query,
        [age, salary, years_of_experience, user_id], // no reference to array
    )?;
    println!(
        "Record with user_id {} updated successfully in table '{}'.",
        user_id, table
    );
    Ok(())
}

/// Deletes record from the specified table based on the user's user_id.
pub fn delete(conn: &Connection, table: &str, user_id: i32) -> Result<()> {
    let delete_query = format!("DELETE FROM {} WHERE user_id = ?1", table);

    // Execute the delete query, user_id is passed directly without reference
    conn.execute(&delete_query, [user_id])?;
    println!(
        "Record with user_id {} deleted successfully from table '{}'.",
        user_id, table
    );
    Ok(())
}

/// Execute read query on the database and prints the result.
pub fn read(conn: &Connection, query: &str) -> Result<()> {
    let mut stmt = conn.prepare(query)?; // Prepare the SQL query
    let rows = stmt.query_map([], |row| {
        let user_id: i32 = row.get(0)?;
        let age: i32 = row.get(1)?;
        let salary: i32 = row.get(2)?;
        let years_of_experience: i32 = row.get(3)?;
        Ok((user_id, age, salary, years_of_experience))
    })?;

    // Iterate through the result set and print each row
    for row in rows {
        let (user_id, age, salary, years_of_experience) = row?;
        println!(
            "user_id: {}, age: {}, salary: {}, years_of_experience: {}",
            user_id, age, salary, years_of_experience
        );
    }

    Ok(())
}
