use diesel::prelude::*;
use crate::orm::schema::newsletters;
use crate::orm::db_connection::*;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde_json::Result;
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Debug, PartialEq)]
pub struct User {
    pub id: String,
    pub email: String,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub fn show() {
    use crate::orm::schema::newsletters::dsl::*;

    let conn = get_establish_connection();

    let results = newsletters.filter(active.eq(true))
        .order(updated_at)
        .limit(5)
        .load::<Newsletter>(&conn)
        .expect("Error loading posts");

    println!("i am here");
    for n in results {
        println!("{}", n.id);
        //newsletterObjects.push(serde_json::to_string(&n))
    }


    //newsletterObjects
}