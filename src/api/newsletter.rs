use actix_web::{web, get, HttpResponse, Responder, Result};
use crate::orm::newsletter::*;
use crate::api::Paginated;

#[get("/newsletters")]
pub async fn newsletters(query: web::Query<Paginated>) -> impl Responder {

    let _newsletters = get_newsletters(query.offset, query.limit);

    //HttpResponse::Ok().finish()
    HttpResponse::Ok().json(_newsletters)
}

#[get("/newsletter/{_id}")]
pub async fn newsletter(_id: web::Path<String, >) -> Result<HttpResponse>  {
    let result = get_newsletter_by_id(_id.to_string()).ok();

    match result {
        // The division was valid
        Some(newsletter) => Ok(HttpResponse::Ok().json(newsletter)),
        // The division was invalid
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
