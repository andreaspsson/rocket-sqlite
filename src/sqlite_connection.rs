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

static DB_STORAGE_NAME: &str = "test-db.db";
static DB_STORAGE_BUCKET: &str = "sqlite-test";
static DB_FILE_LOCATION: &str = "/tmp/test-db.db";

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
    let mut file = File::open(DB_FILE_LOCATION)?;
    file.read_to_end(&mut bytes)?;

    let client = Client::default();
    client
        .object()
        .create(DB_STORAGE_BUCKET, bytes, DB_STORAGE_NAME, "binary/octet-stream")
        .await?;
    Ok(())
}

pub async fn download_db() -> Result<(), SQLiteConnectionError> {
    let client = Client::default();
    let mut stream = client
        .object()
        .download_streamed(DB_STORAGE_BUCKET, DB_STORAGE_NAME)
        .await?;
    let file = File::create(DB_FILE_LOCATION)?;
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
        match sqlite::open(DB_FILE_LOCATION) {
            Ok(sqlte_connection) => Outcome::Success(SQLiteConnection {
                connection: sqlte_connection,
            }),
            Err(err) => Outcome::Failure((Status::InternalServerError, err.into())),
        }
    }
}
