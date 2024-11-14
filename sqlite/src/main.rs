use clap::{Parser, Subcommand};
use rusqlite::{Connection, Result};
use sqlite::*; // Import the functions from lib.rs
// mod mylib; // Declare the mylib module
// use mylib::*; // Import the functions from mylib.rs

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a table
    Create { table_name: String },

    /// Read data from a table
    Read { query: String },

    /// Update a record in a table
    Update {
        table_name: String,
        user_id: i32,
        age: i32,
        salary: i32,
        years_of_experience: i32,
    },

    /// Delete a record from a table
    Delete { table_name: String, user_id: i32 },

    /// Load data from a CSV file into a table
    Load {
        table_name: String,
        file_path: String,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let conn = Connection::open("employee_data.db")?; // Open SQLite database file

    match args.command {
        Commands::Create { table_name } => {
            println!("Creating Table '{}'", table_name);
            create_table(&conn, &table_name).expect("Failed to create table");
        }
        Commands::Read { query } => {
            println!("Executing Query: {}", query);
            read(&conn, &query).expect("Failed to execute query");
        }
        Commands::Update {
            table_name,
            user_id,
            age,
            salary,
            years_of_experience,
        } => {
            println!("Updating Record with user_id {} in table {}", user_id, table_name);
            update(
                &conn,
                &table_name,
                user_id,
                age,
                salary,
                years_of_experience,
            )
            .expect("Failed to update record");
        }
        Commands::Delete { table_name, user_id } => {
            println!("Deleting Record with user_id {} from table {}", user_id, table_name);
            delete(&conn, &table_name, user_id).expect("Failed to delete record");
        }
        Commands::Load {
            table_name,
            file_path,
        } => {
            println!(
                "Loading data into table '{}' from '{}'",
                table_name, file_path
            );
            load_data(&conn, &table_name, &file_path).expect("Failed to load data");
        }
    }
    Ok(())
}
