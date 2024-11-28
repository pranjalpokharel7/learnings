use std::net::TcpListener;

use serde::Deserialize;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};

#[derive(Deserialize)]
struct SubscriptionFormData {
    pub email: String,
    pub name: String,
}

// Responder is a converstion trait into an HttpResponse
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscibe(_form: web::Form<SubscriptionFormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

// std::io::Result<()> is shorthand for Result<(), std::io::Error>.
pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check/", web::get().to(health_check))
            .route("/subscriptions/", web::post().to(subscibe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
