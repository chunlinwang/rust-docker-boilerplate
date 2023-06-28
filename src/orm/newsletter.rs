use diesel::prelude::*;
use crate::orm::schema::newsletters;
use crate::orm::db_connection::*;
use serde::{Serialize, Deserialize };
use chrono::NaiveDateTime;
use diesel::result::Error;
use r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;

// #[derive(Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize)]
// pub struct Newsletter {
//     pub id: String,
//     pub email: String,
//     pub active: bool,
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
// }
//
// pub fn get_newsletter_by_id(_id: String) -> Result<Newsletter, Error> {
//     use crate::orm::schema::newsletters::dsl::*;
//
//     let newsletter: Result<Newsletter, Error> = newsletters.find(&_id).first(&conn);
//
//     newsletter
// }
//
// #[warn(unused_must_use)]
// pub fn get_newsletters(_offset: Option<i64>, _limit: Option<i64>) -> Vec<Newsletter>{
//     use crate::orm::schema::newsletters::dsl::*;
//
//     let mut query_dsl = newsletters.into_boxed();
//
//     if let Some(_offset) = _offset {
//         query_dsl = query_dsl.offset(_offset);
//     }
//
//     if let Some(_limit) = _limit {
//         query_dsl = query_dsl.limit(_limit);
//     }
//
//     let results = query_dsl
//         .order(updated_at)
//         .load::<Newsletter>(&conn)
//         .expect("Error loading posts");
//
//     results
// }