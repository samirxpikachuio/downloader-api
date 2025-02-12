use actix_web::{get, web, App, HttpResponse, HttpServer, Result};
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
    status: String,
}

#[get("/")]
async fn hello() -> Result<HttpResponse> {
    let response = HelloResponse {
        message: "Hello, World!".to_string(),
        status: "success".to_string(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[get("/hello/{name}")]
async fn hello_name(name: web::Path<String>) -> Result<HttpResponse> {
    let response = HelloResponse {
        message: format!("Hello, {}!", name),
        status: "success".to_string(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_name)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}