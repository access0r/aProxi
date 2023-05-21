use actix_web::{web, HttpResponse, Responder};

pub async fn endpoint1() -> impl Responder {
    match perform_action() {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

pub async fn endpoint2() -> impl Responder {
    match perform_action() {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

pub async fn endpoint3() -> impl Responder {
    match perform_action() {
        Ok(result) => HttpResponse::Ok().body(result),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}
