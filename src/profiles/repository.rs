#![allow(proc_macro_derive_resolution_fallback)]

use crate::profiles::{Profile, InsertableCat};
use crate::mongo_connection::Conn;
use crate::r2d2_mongodb::mongodb::db::ThreadedDatabase;
use mongodb::{bson, coll::results::DeleteResult, doc, error::Error, oid::ObjectId};

const COLLECTION: &str = "profile";
