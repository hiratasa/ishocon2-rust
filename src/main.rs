mod candidate;
mod helpers;

use actix_files::Files;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde::Serialize;

use candidate::*;

#[derive(Serialize)]
struct SexRatio {
    men: i32,
    women: i32,
}

#[derive(Serialize)]
struct IndexTmplContext {
    candidates: Vec<CandidateElectionResult>,
    parties: Vec<PartyElectionResult>,
    sex_ratio: SexRatio,
}

async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    // TODO:
    //  - Display TOP10 and the worst.
    //  - Display result for each party, by descending order.
    //  - Display ratio for each gender.
    let data = IndexTmplContext {
        candidates: vec![CandidateElectionResult {
            id: 0,
            name: "dummy".to_owned(),
            political_party: "dummy_party".to_owned(),
            sex: "".to_owned(),
            vote_count: 0,
        }],
        parties: vec![PartyElectionResult {
            political_party: "dummy_party".to_owned(),
            vote_count: 0,
        }],
        sex_ratio: SexRatio { men: 0, women: 0 },
    };

    HttpResponse::Ok().body(hb.render("index", &data).unwrap())
}

#[derive(Serialize)]
struct CandidateTmplContext {
    candidate: Candidate,
    votes: i32,
    keywords: Vec<String>,
}

async fn show_candidate(
    hb: web::Data<Handlebars<'_>>,
    _id: web::Path<(String,)>,
) -> impl Responder {
    // TODO:
    //  - Display vote count.
    //  - Display voice of supportes.
    let data = CandidateTmplContext {
        candidate: Candidate {
            id: 0,
            name: "dummy".to_owned(),
            political_party: "dummy_party".to_owned(),
            sex: "".to_owned(),
        },
        votes: 0,
        keywords: vec!["dummy_keyword".to_owned()],
    };

    HttpResponse::Ok().body(hb.render("candidate", &data).unwrap())
}

#[derive(Serialize)]
struct PoliticalPartyTmplContext {
    political_party: String,
    votes: i32,
    candidates: Vec<Candidate>,
    keywords: Vec<String>,
}

async fn show_political_party(
    hb: web::Data<Handlebars<'_>>,
    _name: web::Path<(String,)>,
) -> impl Responder {
    // TODO:
    //  - Display vote count for the party.
    //  - Display candidates of the party.
    //  - Display voice of supporters of the candidates.
    let data = PoliticalPartyTmplContext {
        political_party: "".to_owned(),
        votes: 0,
        candidates: vec![Candidate {
            id: 0,
            name: "".to_owned(),
            political_party: "".to_owned(),
            sex: "".to_owned(),
        }],
        keywords: vec!["dummy_keyword".to_owned()],
    };

    HttpResponse::Ok().body(hb.render("political_party", &data).unwrap())
}

#[derive(Serialize)]
struct VoteTmplContext {
    candidates: Vec<Candidate>,
    message: String,
}

async fn show_vote(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    // TODO: Display all candidates
    let data = VoteTmplContext {
        candidates: vec![Candidate {
            id: 0,
            name: "".to_owned(),
            political_party: "".to_owned(),
            sex: "".to_owned(),
        }],
        message: String::new(),
    };

    HttpResponse::Ok().body(hb.render("vote", &data).unwrap())
}

async fn do_vote(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    // TODO:
    //  - Fetch form values.
    //  - Validate user info, user vote upper bound, candidate name and keyword.
    //  - Execute vote (insert to DB)
    let data = VoteTmplContext {
        candidates: vec![Candidate {
            id: 0,
            name: "".to_owned(),
            political_party: "".to_owned(),
            sex: "".to_owned(),
        }],
        message: String::new(),
    };

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
