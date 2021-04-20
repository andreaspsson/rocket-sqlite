use cloud_storage::Client;
use cloud_storage::Error as StorageError;
use futures::StreamExt;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use sqlite::{Connection, Error as SQLiteError};
use std::fs::File;
use std::io::{BufWriter, Error as IOError, Read, Write};
use thiserror::Error;

pub struct SQLiteConnection {
    pub connection: Connection,
}

static DB_PATH: &str = "/tmp/test-db.db";

#[derive(Error, Debug)]
pub enum SQLiteConnectionError {
    #[error(transparent)]
    StorageError(#[from] StorageError),
    #[error(transparent)]
    SQLiteError(#[from] SQLiteError),
    #[error(transparent)]
    IOError(#[from] IOError),
}

// TODO - stream upload instad of readin full file into memory
pub async fn upload_db() -> Result<(), SQLiteConnectionError> {
    let mut bytes: Vec<u8> = Vec::new();
    let mut file = File::open(DB_PATH)?;
    file.read_to_end(&mut bytes)?;

    let client = Client::default();
    client
        .object()
        .create("sqlite-test", bytes, "test-db.db", "binary/octet-stream")
        .await?;
    Ok(())
}

pub async fn download_db() -> Result<(), SQLiteConnectionError> {
    let client = Client::default();
    let mut stream = client
        .object()
        .download_streamed("sqlite-test", "test-db.db")
        .await?;
    let file = File::create(DB_PATH)?;
    let mut file_writer = BufWriter::new(file);

    while let Some(byte) = stream.next().await {
        file_writer.write_all(&[byte.unwrap()]).unwrap();
    }
    Ok(())
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SQLiteConnection {
    type Error = SQLiteConnectionError;

    async fn from_request(_req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match sqlite::open(DB_PATH) {
            Ok(sqlte_connection) => Outcome::Success(SQLiteConnection {
                connection: sqlte_connection,
            }),
            Err(err) => Outcome::Failure((Status::InternalServerError, err.into())),
        }
    }
}
