use diesel::prelude::*;
use crate::orm::schema::newsletters;
use crate::orm::db_connection::*;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::Result;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Debug, PartialEq)]
pub struct Image {
    pub id: String,
    pub hash: String,
}

pub fn show() {}