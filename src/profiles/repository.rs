#![allow(proc_macro_derive_resolution_fallback)]

use crate::profiles::{Profile, InsertableProfile};
use crate::mongo_connection::Conn;
use crate::r2d2_mongodb::mongodb::db::ThreadedDatabase;
use mongodb::{bson, coll::results::DeleteResult, doc, error::Error, oid::ObjectId};

const COLLECTION: &str = "profile";

pub fn all(connection: &Conn) -> Result<Vec<Profile>, Error> {
    let cursor = connection.collection(COLLECTION).find(None, None).unwrap();

    cursor
        .map(|result| match result {
            Ok(doc) => match bson::from_bson(bson::Bson::Document(doc)) {
                Ok(result_model) => Ok(result_model),
                Err(_) => Err(Error::DefaultError(String::from(""))),
            },
            Err(err) => Err(err),
        })
        .collect::<Result<Vec<Profile>, Error>>()
}