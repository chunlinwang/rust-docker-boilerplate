use actix_web::{get, HttpResponse, Responder};

#[get("/purchases")]
pub async fn purchases() -> impl Responder {
        HttpResponse::Ok().body("Hello world! purchases")
    }

#[get("/purchase/{id}")]
pub async fn purchase() -> impl Responder {
    HttpResponse::Ok().body("Hello world! purchases")
}

