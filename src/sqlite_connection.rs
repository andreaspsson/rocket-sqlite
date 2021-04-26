use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use sqlite::{Connection, Error as SQLiteError};
use std::env::{var, VarError};
use thiserror::Error;

pub struct SQLiteConnection {
    pub connection: Connection,
}

#[derive(Error, Debug)]
pub enum SQLiteConnectionError {
    #[error(transparent)]
    SQLiteError(#[from] SQLiteError),
    #[error(transparent)]
    ConfigError(#[from] VarError),
}

fn open_db_connection() -> Result<SQLiteConnection, SQLiteConnectionError> {
    let file_name = var("SQLITE_DB_FILE")?;
    let connection = sqlite::open(file_name)?;
    Ok(SQLiteConnection {
        connection,
    })
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for SQLiteConnection {
    type Error = SQLiteConnectionError;

    async fn from_request(_req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match open_db_connection() {
            Ok(connection) => Outcome::Success(connection),
            Err(err) => Outcome::Failure((Status::InternalServerError, err.into())),
        }
    }
}
