use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    // TODO:
    //  - Display TOP10 and the worst.
    //  - Display result for each party, by descending order.
    //  - Display ratio for each gender.
    HttpResponse::Ok().body("Hello!")
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
    })
    .bind("localhost:8080")?
    .run()
    .await
}
