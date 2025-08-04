use actix_web::{HttpResponse, Responder, get};

#[get("")]
pub async fn user_root() -> impl Responder {
    HttpResponse::Ok().body("user root")
}

#[get("/hey")]
pub async fn user_hey() -> impl Responder {
    HttpResponse::Ok().body("user hey")
}
