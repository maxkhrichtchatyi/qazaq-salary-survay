pub mod handler;
pub mod repository;

use mongodb::bson;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    #[serde(rename = "_id")]  // Use MongoDB's special primary key field name when serializing
    pub id: Option<bson::oid::ObjectId>,
    pub name: Option<String>,
    pub age: Option<i8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InsertableProfile {
    pub name: Option<String>,
    pub age: Option<i8>,
}

impl InsertableProfile {
    fn from_cat(cats: Cat) -> InsertableCat {
        InsertableCat {
            name: cats.name,
            age: cats.age,
        }
    }
}
