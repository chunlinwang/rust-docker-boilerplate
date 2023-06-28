use actix_web::{get, HttpResponse, Responder};

#[get("/products")]
pub async fn users() -> impl Responder {
        HttpResponse::Ok().body("Hello world! products")
    }

