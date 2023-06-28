use actix_web::{web, get, HttpResponse, Responder};

#[get("/image/{hash}")]
pub async fn image(web::Path(_hash): web::Path<String>) -> impl Responder {
    let mut resposne = String::from("Hello world! image");
    resposne.push_str(_hash.as_str());

    HttpResponse::Ok().body(resposne)
}

