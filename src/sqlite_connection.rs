use cloud_storage::Client;
use cloud_storage::Error as StorageError;
use futures::StreamExt;
use reqwest::StatusCode;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use sqlite::{Connection, Error as SQLiteError};
use std::env::{var, VarError};
use std::fs::File;
use std::io::{BufWriter, Error as IOError, Read, Write};
use thiserror::Error;

pub struct SQLiteConnection {
    pub connection: Connection,
}

static BUCKET_NAME_VAR: &str = "BUCKET_NAME";
static DB_STORAGE_NAME: &str = "test-db.db";
static DB_FILE_LOCATION: &str = "/tmp/test-db.db";

#[derive(Error, Debug)]
pub enum SQLiteConnectionError {
    #[error(transparent)]
    StorageError(#[from] StorageError),
    #[error(transparent)]
    SQLiteError(#[from] SQLiteError),
    #[error(transparent)]
    IOError(#[from] IOError),
    #[error(transparent)]
    ConfigError(#[from] VarError),
}

// TODO - stream upload instad of readin full file into memory
pub async fn upload_db() -> Result<(), SQLiteConnectionError> {
    let mut bytes: Vec<u8> = Vec::new();
    let bucket_name = var(BUCKET_NAME_VAR)?;
    let mut file = File::open(DB_FILE_LOCATION)?;
    file.read_to_end(&mut bytes)?;

    let client = Client::default();
    client
        .object()
        .create(&bucket_name, bytes, DB_STORAGE_NAME, "binary/octet-stream")
        .await?;
    Ok(())
}

pub async fn download_db() -> Result<(), SQLiteConnectionError> {
    let bucket_name = var(BUCKET_NAME_VAR)?;
    let client = Client::default();
    let stream_request = client
        .object()
        .download_streamed(&bucket_name, DB_STORAGE_NAME)
        .await;
    match stream_request {
        Ok(mut stream) => {
            let file = File::create(DB_FILE_LOCATION)?;
            let mut file_writer = BufWriter::new(file);

            while let Some(byte) = stream.next().await {
                file_writer.write_all(&[byte.unwrap()]).unwrap();
            }
            Ok(())
        }
        Err(err) => match err {
            StorageError::Reqwest(ref request_err) => {
                if let Some(status) = request_err.status() {
                    if status == StatusCode::NOT_FOUND {
                        println!("File not found in bucket, skipping download");
                        Ok(())
                    } else {
                        Err(err.into())
                    }
                } else {
                    Err(err.into())
                }
            }
            _ => Err(err.into()),
        },
    }
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
