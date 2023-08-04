use mongodb::bson;
use std::convert::Infallible;
use thiserror::Error;
use warp::{http::StatusCode, reply, Rejection, Reply};

use crate::response::GenericResponse;

#[derive(Error, Debug)]
pub enum Error {
    #[error("mongodb error: {0}")]
    MongoError(#[from] mongodb::error::Error),
    #[error("error during query: {0}")]
    MongoQueryError(mongodb::error::Error),
    #[error("duplicate key error: {0}")]
    MongoDuplicateError(mongodb::error::Error),
    #[error("serialization failed: {0}")]
    MongoSerializeBsonError(bson::ser::Error),
    #[error("could not access field in document: {0}")]
    MongoDataError(#[from] bson::document::ValueAccessError),
    #[error("invalid id used: {0}")]
    InvalidIDError(String),
}

impl warp::reject::Reject for Error {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<Box<dyn Reply>, Infallible> {
    let code;
    let message;
    let status;

    if err.is_not_found() {
        status = "failed";
        code = StatusCode::NOT_FOUND;
        message = "Route does not exist on server";
    } else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
        status = "failed";
        code = StatusCode::BAD_REQUEST;
        message = "Invalid Body";
    } else if let Some(e) = err.find::<Error>() {
        match e {
            Error::MongoError(e) => {
                eprintln!("MongoDB error: {:?}", e);
                status = "fail";
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "mongodb error"
            }
            Error::MongoDuplicateError(e) => {
                eprintln!("MongoDB error: {:?}", e);
                status = "fail";
                code = StatusCode::CONFLICT;
                message = "Duplicate key error";
            }
            Error::MongoQueryError(e) => {
                eprintln!("Error during mongodb query: {:?}", e);
                status = "fail";
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Error during mongodb query";
            }
            Error::MongoSerializeBsonError(e) => {
                eprintln!("Error seserializing BSON: {:?}", e);
                status = "fail";
                code = StatusCode::INTERNAL_SERVER_ERROR;
                message = "Error seserializing BSON";
            }
            Error::MongoDataError(e) => {
                eprintln!("validation error: {:?}", e);
                status = "fail";
                code = StatusCode::BAD_REQUEST;
                message = "validation error";
            }
            Error::InvalidIDError(e) => {
                eprintln!("Invalid ID: {:?}", e);
                status = "fail";
                code = StatusCode::BAD_REQUEST;
                message = e.as_str();
            }
        }
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        status = "failed";
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Method not allowed";
    } else {
        eprintln!("unhandled error: {:?}", err);
        status = "error";
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal Server error";
    }

    let json = reply::json(&GenericResponse {
        status: status.into(),
        message: message.into(),
    });

    Ok(Box::new(reply::with_status(json, code)))
}
