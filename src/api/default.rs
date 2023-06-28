use actix_web::{get, Responder, web, HttpResponse};
use actix_identity::Identity;

#[get("/")]
pub async fn index(user: Option<Identity>) -> impl Responder {
    if let Some(user) = user {
        println!("Welcome! {:?}", user.id().unwrap());
    } else {
        println!("Welcome Anonymous!");
    }

    web::Json(
        [1, 2, 3]
        )
}