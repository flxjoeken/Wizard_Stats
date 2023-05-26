mod db_utils;

use crate::db_utils::create_tables;

fn main() {
    let connection = match db_utils::create_connection() {
        Ok(connection) => connection,
        Err(_) => {
            println!("An error occurred creating the connection");
            return;
        }
    };
    create_tables(&connection);
}
