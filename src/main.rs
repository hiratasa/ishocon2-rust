mod helpers;

use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde_json::json;

async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    // TODO:
    //  - Display TOP10 and the worst.
    //  - Display result for each party, by descending order.
    //  - Display ratio for each gender.
    let data = json!({
        "candidates": [
            {
                "id": 0,
                "name": "dummy",
                "vote_count": 0,
                "political_party": "dummy_party",
            }
        ],
        "parties": [
            {
                "political_party": "dummy_party",
                "vote_count": 0,
            }
        ],
        "sex_ratio": {
            "men": 0,
            "women": 0,
        },
    });

    HttpResponse::Ok().body(hb.render("index", &data).unwrap())
}

async fn show_candidate(
    hb: web::Data<Handlebars<'_>>,
    _id: web::Path<(String,)>,
) -> impl Responder {
    // TODO:
    //  - Display vote count.
    //  - Display voice of supportes.
    let data = json!({
        "candidate": {
            "name": "dummy",
            "political_party": "dummy_party",
            "sex": "",
        },
        "votes": 0,
        "keywords": ["dummy_keyword"],
    });

    HttpResponse::Ok().body(hb.render("candidate", &data).unwrap())
}

async fn show_political_party(
    hb: web::Data<Handlebars<'_>>,
    _name: web::Path<(String,)>,
) -> impl Responder {
    // TODO:
    //  - Display vote count for the party.
    //  - Display candidates of the party.
    //  - Display voice of supporters of the candidates.
    let data = json!({
        "political_party": "",
        "votes": 0,
        "candidates": [
            {
                "name": "dummy",
            }
        ],
        "keywords": ["dummy_keyword"],
    });

    HttpResponse::Ok().body(hb.render("political_party", &data).unwrap())
}

async fn show_vote(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    // TODO: Display all candidates
    let data = json!({
        "candidates": [
            {
                "name": "dummy",
            }
        ],
        "message": "",
    });

    HttpResponse::Ok().body(hb.render("vote", &data).unwrap())
}

async fn do_vote(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    // TODO:
    //  - Fetch form values.
    //  - Validate user info, user vote upper bound, candidate name and keyword.
    //  - Execute vote (insert to DB)
    let data = json!({
        "candidates": [
            {
                "name": "dummy",
            }
        ],
        "message": "",
    });

    HttpResponse::Ok().body(hb.render("vote", &data).unwrap())
}

async fn initialize() -> impl Responder {
    // TODO:
    //  - Delete all votes.
    "Finish"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // TODO:
    //  - prepare DB
    //  - prepare session

    let mut hb = Handlebars::new();
    hb.register_templates_directory(".hbs", "./templates/")
        .unwrap();
    hb.register_helper("plus1", Box::new(helpers::plus1));
    let hb = web::Data::new(hb);

    HttpServer::new(move || {
        App::new()
            .app_data(hb.clone())
            .service(Files::new("/css", "./public/css"))
            .route("/", web::get().to(index))
            .route("/candidates/{id}", web::get().to(show_candidate))
            .route(
                "/political_parties/{name}",
                web::get().to(show_political_party),
            )
            .route("/vote", web::get().to(show_vote))
            .route("/vote", web::post().to(do_vote))
            .route("/initialize", web::get().to(initialize))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
