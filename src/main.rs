use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/css", "./public/css"))
            .route("/", web::get().to(index))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
