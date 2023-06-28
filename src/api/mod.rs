pub mod default;
pub mod identification;
//pub mod user;
//pub mod category;
//pub mod newsletter;
//pub mod product;
//pub mod purchase;
//pub mod image;
//pub mod address;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Paginated {
    offset: Option<i64>,
    limit: Option<i64>,
}