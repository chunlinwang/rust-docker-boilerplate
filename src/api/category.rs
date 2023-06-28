use actix_web::{get, HttpResponse, Responder};

#[get("/categories")]
pub async fn users() -> impl Responder {
    HttpResponse::Ok().body("Hello world! categories")
}

