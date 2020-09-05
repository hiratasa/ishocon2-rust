use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    // TODO:
    //  - Display TOP10 and the worst.
    //  - Display result for each party, by descending order.
    //  - Display ratio for each gender.
    HttpResponse::Ok().body("Hello!")
}

async fn show_candidate(id: web::Path<(String,)>) -> impl Responder {
    // TODO:
    //  - Display vote count.
    //  - Display voice of supportes.
    id.0.clone()
}

async fn show_political_party(name: web::Path<(String,)>) -> impl Responder {
    name.0.clone()
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // TODO:
    //  - prepare DB
    //  - prepare session
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/css", "./public/css"))
            .route("/", web::get().to(index))
            .route("/candidates/{id}", web::get().to(show_candidate))
            .route(
                "/political_parties/{name}",
                web::get().to(show_political_party),
            )
    })
    .bind("localhost:8080")?
    .run()
    .await
}
