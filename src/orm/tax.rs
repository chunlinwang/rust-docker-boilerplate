use crate::orm::schema::newsletters;
use crate::orm::db_connection::*;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::Result;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Debug, PartialEq)]
pub struct Tax {
    pub id: String,
    pub email: String,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
