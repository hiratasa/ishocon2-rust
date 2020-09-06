mod candidate;
mod helpers;
mod newrelic_util;
mod user;
mod vote;

use std::cmp::Reverse;
use std::collections::HashMap;
use std::env;

use actix_files::Files;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

use candidate::*;
use user::*;
use vote::*;

use newrelic_util::NewRelicAppData;

#[derive(Serialize)]
struct SexRatio {
    men: i64,
    women: i64,
}

#[derive(Serialize)]
struct IndexTmplContext {
    candidates: Vec<CandidateElectionResult>,
    parties: Vec<PartyElectionResult>,
    sex_ratio: SexRatio,
}

async fn index(
    pool: web::Data<MySqlPool>,
    hb: web::Data<Handlebars<'_>>,
    newrelic: web::Data<NewRelicAppData>,
) -> impl Responder {
    let _transaction = newrelic.transaction("GET index");

    let election_results = get_election_result(&pool).await;

    let tmp = election_results.clone();
    let mut candidates = vec![];
    candidates.extend_from_slice(&tmp[0..10]);
    candidates.push(tmp.last().unwrap().clone());

    let party_names = get_all_party_name(&pool).await;
    let mut party_result_map = HashMap::new();
    for party_name in party_names {
        party_result_map.insert(party_name, 0);
    }
    for r in &election_results {
        *party_result_map.get_mut(&r.political_party).unwrap() += r.vote_count;
    }
    let mut parties = vec![];
    for (political_party, vote_count) in party_result_map {
        parties.push(PartyElectionResult {
            political_party,
            vote_count,
        });
    }
    parties.sort_unstable_by_key(|r| Reverse(r.vote_count));

    let mut sex_ratio = SexRatio { men: 0, women: 0 };
    for r in &election_results {
        if r.sex == "男" {
            sex_ratio.men += r.vote_count;
        } else if r.sex == "女" {
            sex_ratio.women += r.vote_count;
        }
    }

    let data = IndexTmplContext {
        candidates,
        parties,
        sex_ratio,
    };

    HttpResponse::Ok().body(hb.render("index", &data).unwrap())
}

#[derive(Serialize)]
struct CandidateTmplContext {
    candidate: Candidate,
    votes: i64,
    keywords: Vec<String>,
}

async fn show_candidate(
    pool: web::Data<MySqlPool>,
    hb: web::Data<Handlebars<'_>>,
    path: web::Path<(i32,)>,
    newrelic: web::Data<NewRelicAppData>,
) -> impl Responder {
    let _transaction = newrelic.transaction("GET candidate");

    let id = path.0;
    let candidate = match get_candidate(&pool, id).await {
        Some(candidate) => candidate,
        None => return HttpResponse::Found().header(header::LOCATION, "/").finish(),
    };
    let votes = get_vote_count_by_candidate_id(&pool, id).await;
    let keywords = get_voice_of_supporter(&pool, &vec![id]).await;

    let data = CandidateTmplContext {
        candidate,
        votes,
        keywords,
    };

    HttpResponse::Ok().body(hb.render("candidate", &data).unwrap())
}

#[derive(Serialize)]
struct PoliticalPartyTmplContext {
    political_party: String,
    votes: i64,
    candidates: Vec<Candidate>,
    keywords: Vec<String>,
}

async fn show_political_party(
    pool: web::Data<MySqlPool>,
    hb: web::Data<Handlebars<'_>>,
    path: web::Path<(String,)>,
    newrelic: web::Data<NewRelicAppData>,
) -> impl Responder {
    let _transaction = newrelic.transaction("GET political_party");

    let political_party = &path.0;
    let election_results = get_election_result(&pool).await;
    let mut votes = 0;
    for r in election_results {
        if &r.political_party == political_party {
            votes += r.vote_count;
        }
    }

    let candidates = get_candidates_by_political_party(&pool, political_party).await;
    let mut candidate_ids = vec![];
    for c in &candidates {
        candidate_ids.push(c.id);
    }

    let keywords = get_voice_of_supporter(&pool, &candidate_ids).await;

    let data = PoliticalPartyTmplContext {
        political_party: political_party.clone(),
        votes,
        candidates,
        keywords,
    };

    HttpResponse::Ok().body(hb.render("political_party", &data).unwrap())
}

#[derive(Serialize)]
struct VoteTmplContext {
    candidates: Vec<Candidate>,
    message: String,
}

async fn show_vote(
    pool: web::Data<MySqlPool>,
    hb: web::Data<Handlebars<'_>>,
    newrelic: web::Data<NewRelicAppData>,
) -> impl Responder {
    let _transaction = newrelic.transaction("GET vote");

    let candidates = get_all_candidate(&pool).await;

    let data = VoteTmplContext {
        candidates,
        message: String::new(),
    };

    HttpResponse::Ok().body(hb.render("vote", &data).unwrap())
}

#[derive(Deserialize)]
struct VoteFormData {
    name: String,
    address: String,
    mynumber: String,
    candidate: String,
    vote_count: i64,
    keyword: String,
}

async fn do_vote(
    pool: web::Data<MySqlPool>,
    hb: web::Data<Handlebars<'_>>,
    form: web::Form<VoteFormData>,
    newrelic: web::Data<NewRelicAppData>,
) -> impl Responder {
    let _transaction = newrelic.transaction("POST vote");

    let user = get_user(&pool, &form.name, &form.address, &form.mynumber).await;
    let candidate = get_candidate_by_name(&pool, &form.candidate).await;
    let voted_count = get_user_voted_count(&pool, user.as_ref().map_or(0, |u| u.id)).await;
    let candidates = get_all_candidate(&pool).await;
    let vote_count = form.vote_count;

    let message = match user {
        None => "個人情報に誤りがあります",
        Some(user) => {
            if (user.votes as i64) < vote_count + voted_count {
                "投票数が上限を超えています"
            } else if form.candidate == "" {
                "候補者を記入してください"
            } else if candidate.is_none() {
                "候補者を正しく記入してください"
            } else if form.keyword == "" {
                "投票理由を記入してください"
            } else {
                for _ in 0..vote_count {
                    create_vote(
                        &pool,
                        user.id,
                        candidate.as_ref().unwrap().id,
                        &form.keyword,
                    )
                    .await;
                }
                "投票に成功しました"
            }
        }
    };

    let data = VoteTmplContext {
        candidates,
        message: message.to_owned(),
    };

    HttpResponse::Ok().body(hb.render("vote", &data).unwrap())
}

async fn initialize(pool: web::Data<MySqlPool>) -> impl Responder {
    sqlx::query!("DELETE FROM votes")
        .execute(pool.get_ref())
        .await
        .expect("failed to initialize.");

    HttpResponse::Ok().body("Finish")
}

fn database_url() -> String {
    let user = env::var("ISHOCON2_DB_USER").unwrap_or("ishocon".to_owned());
    let pass = env::var("ISHOCON2_DB_PASSWORD").unwrap_or("ishocon".to_owned());
    let dbname = env::var("ISHOCON2_DB_NAME").unwrap_or("ishocon2".to_owned());

    format!("mysql://{}:{}@localhost/{}", user, pass, dbname)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url())
        .await
        .unwrap();
    let pool = web::Data::new(pool);

    let mut hb = Handlebars::new();
    hb.register_templates_directory(".hbs", "./templates/")
        .unwrap();
    hb.register_helper("plus1", Box::new(helpers::plus1));
    let hb = web::Data::new(hb);

    let newrelic = web::Data::new(newrelic_util::create_app());

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .app_data(hb.clone())
            .app_data(newrelic.clone())
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
