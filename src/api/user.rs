use actix_web::{web, get, put, post, error, HttpRequest, HttpResponse, Responder, Error};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    lastname: String,
}

#[get("/users")]
pub async fn users() -> impl Responder {
        HttpResponse::Ok().body("Hello world! users")
    }

#[post("/user")]
pub async fn create( payload: web::Payload) -> Result<HttpResponse, Error> {
    // // payload is a stream of Bytes objects
    // let mut body = web::BytesMut::new();
    //
    // while let Some(item) = payload.next().await {
    //     body.extend_from_slice(&item?);
    // }
    //
    // // body is loaded, now we can deserialize serde-json
    // let obj = serde_json::from_slice::<User>(&body)?;
    let obj = String::from("{}");
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

#[put("/user/{id}")]
pub async fn update(id: web::Path<String>, payload: web::Payload) -> Result<HttpResponse, Error> {
    // let user_id = id;
    // let mut body = web::BytesMut::new();
    //
    // while let Some(item) = payload.next().await {
    //     body.extend_from_slice(&item?);
    // }
    //
    // // body is loaded, now we can deserialize serde-json
    // let obj = serde_json::from_slice::<User>(&body)?;
    let obj = String::from("{}");
    Ok(HttpResponse::Ok().json(obj)) // <- send response
}

