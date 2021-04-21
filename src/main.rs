mod sqlite_connection;
use sqlite::{Error as ErrorSQLite};
use sqlite_connection::{download_db, upload_db, SQLiteConnection};
use rocket::http::Status;
use rocket::response::status;

#[macro_use]
extern crate rocket;

struct SQLiteError(ErrorSQLite);

impl From<ErrorSQLite> for SQLiteError {
    fn from(error: ErrorSQLite) -> Self {
        SQLiteError(error)
    }
}

impl From<SQLiteError> for status::Custom<String> {
    fn from(error: SQLiteError) -> Self {
        let error_message = error.0.message
            .unwrap_or(String::from("database error"));
        status::Custom(Status::InternalServerError, error_message)
    }
}

fn do_get_count(connection: &SQLiteConnection) -> Result<String, SQLiteError> {
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
async fn get_count(connection: SQLiteConnection) -> Result<String, status::Custom<String>> {
    let count = do_get_count(&connection)?;
    Ok(count)
}

fn do_insert(connection: &SQLiteConnection, _message: String) -> Result<(), SQLiteError> {
    connection
        .connection
        .execute("INSERT INTO test_request VALUES (NULL, 'TODO insert a string instead!');")?;
    Ok(())
}

/**
 * Insert item into database
 */
#[post("/")]
async fn insert_item(connection: SQLiteConnection) -> Result<String, status::Custom<String>> {
    do_insert(&connection, String::from("message"))?;
    Ok(String::from("item inserted"))
}

/**
 *
 */
#[post("/reset")]
async fn reset_db(connection: SQLiteConnection) -> Result<String, status::Custom<String>> {
    let reset_res = connection
        .connection
        .execute("
            DROP TABLE IF EXISTS test_request;
            CREATE TABLE test_request (row_id INTEGER PRIMARY KEY, message TEXT);
        ");
    match reset_res {
        Ok(_) => Ok(String::from("reset done")),
        Err(err) => Err(SQLiteError(err).into()),
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
