use std::fmt::Pointer;

use actix_web::{FromRequest, HttpResponse};
use actix_web::web::Json;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    message: String
}

pub async fn complain(form: Json<FormData>) -> HttpResponse {
    let message = form.0.message;
    println!("{message}");
    HttpResponse::Ok().finish()
}