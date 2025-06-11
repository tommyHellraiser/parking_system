use crate::api::STOP_SENDER;
use actix_web::{HttpResponse, get, put, web::ServiceConfig};

pub(in super::super) fn service_router(cfg: &mut ServiceConfig) {
    cfg.service(alive).service(stop);
}

#[get("alive")]
async fn alive() -> HttpResponse {
    HttpResponse::Ok().body("I'm alive!")
}

#[put("stop")]
async fn stop() -> HttpResponse {
    let Some(stop_sender) = STOP_SENDER.get() else {
        return HttpResponse::InternalServerError().finish();
    };
    if let Err(error) = stop_sender.send(()).await {
        println!("Error stopping http server: {}", error);
    };

    HttpResponse::Ok().body("Stopping server...")
}
