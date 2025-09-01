use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("debug-app is running")
}

#[get("/healthz")]
async fn healthz() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    println!("Starting debug-app on {}", addr);

    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(healthz)
    })
    .bind(addr)?
    .run()
    .await
}
