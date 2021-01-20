use crate::profiles;
use crate::mongo_connection::Conn;
use cats::Cat;
use mongodb::{doc, error::Error, oid::ObjectId};
use rocket_contrib::json::Json;
use rocket::{http::Status};

fn error_status(error: Error) -> Status {
    match error {
        Error::CursorNotFoundError => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

#[get("/")]
pub fn all(connection: Conn) -> Result<Json<Vec<Cat>>, Status> {
    match cats::repository::all(&connection) {
        Ok(res) => Ok(Json(res)),
        Err(err) => Err(error_status(err)),
    }
}