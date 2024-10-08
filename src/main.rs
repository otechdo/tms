use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/hello")] // Endpoint pour une requête GET sur /hello
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
