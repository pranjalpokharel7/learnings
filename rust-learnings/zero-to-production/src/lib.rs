use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[allow(dead_code)]
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

// Responder is a converstion trait into an HttpResponse
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

// std::io::Result<()> is shorthand for Result<(), std::io::Error>.
pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server =
        HttpServer::new(|| App::new().route("/health_check/", web::get().to(health_check)))
            .listen(listener)?
            .run();

    Ok(server)
}
