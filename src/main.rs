mod sqlite_connection;
use sqlite::Error;
use sqlite_connection::{download_db, upload_db, SQLiteConnection};

#[macro_use]
extern crate rocket;

fn do_get_count(connection: &SQLiteConnection) -> Result<String, Error> {
    let mut statement = connection
        .connection
        .prepare("SELECT row_id FROM test_request ORDER BY row_id DESC LIMIT 1;")?;
    statement.next()?; // Fetch first row in result
    let content = statement.read::<i64>(0)?;
    Ok(content.to_string())
}

/**
 * Fetch current row count, or no it actually returns the max value of an auto incrementing integer key
 */
#[get("/")]
async fn get_count(connection: SQLiteConnection) -> String {
    match do_get_count(&connection) {
        Ok(res) => res,
        Err(err) => {
            println!("{:?}", err);
            err.message
                .unwrap_or(String::from("failed to get item count"))
        }
    }
}

fn do_insert(connection: &SQLiteConnection, _message: String) -> Result<(), Error> {
    connection
        .connection
        .execute("INSERT INTO test_request VALUES (NULL, 'TODO insert a string instead!');")?;
    Ok(())
}

/**
 * Insert item into database
 */
#[post("/")]
async fn insert_item(connection: SQLiteConnection) -> String {
    match do_insert(&connection, String::from("message")) {
        Ok(_) => String::from("insert done"),
        Err(err) => {
            println!("{:?}", err);
            err.message.unwrap_or(String::from("failed to insert item"))
        }
    }
}

/**
 *
 */
#[post("/reset")]
async fn reset_db(connection: SQLiteConnection) -> String {
    let reset_res = connection
        .connection
        .execute("
            DROP TABLE IF EXISTS test_request;
            CREATE TABLE test_request (row_id INTEGER PRIMARY KEY, message TEXT);
        ");
    match reset_res {
        Ok(_) => String::from("reset done"),
        Err(err) => {
            println!("{:?}", err);
            err.message
                .unwrap_or(String::from("failed to execute statement"))
        }
    }
}

#[rocket::main]
async fn main() {
    // Download db file
    match download_db().await {
        Err(err) => {
            println!("Error when downloading file {:?}", err);
            panic!("Failed to download db file");
        }
        Ok(_) => {
            println!("Download done!");
        }
    }
    // Launch server
    let rocket_res = rocket::build()
        .mount("/work", routes![get_count, insert_item])
        .mount("/config", routes![reset_db])
        .launch()
        .await;

    println!("Closing down, fist upload db to storage...");
    match rocket_res {
        Err(err) => {
            println!("Error when closing down rocket {:?}", err);
        }
        _ => {} // All good, ignore
    }
    // Download db file
    match upload_db().await {
        Err(err) => {
            println!("Error when downloading file {:?}", err);
            panic!("Failed to upload db file");
        }
        _ => {
            println!("Upload done done!");
        }
    }
}
