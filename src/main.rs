mod candidate;
mod user;
mod vote;

#[macro_use]
mod newrelic_util;

use std::cmp::Reverse;
use std::collections::HashMap;
use std::env;

use actix_files::Files;
use actix_web::{http::header, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use futures::join as futures_join;
use serde::Deserialize;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

use candidate::*;
use user::*;
use vote::*;

struct SexRatio {
    men: i32,
    women: i32,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTmplContext<'a> {
    top_candidates: &'a [Candidate],
    worst_candidate: &'a Candidate,
    parties: Vec<PartyElectionResult>,
    sex_ratio: SexRatio,
}

async fn index(pool: web::Data<MySqlPool>) -> impl Responder {
    newrelic_transaction!("GET index");

    let candidates = get_all_candidate_sorted(&pool).await;

    let party_names = get_all_party_name(&pool).await;
    let mut party_result_map = HashMap::new();
    for party_name in party_names {
        party_result_map.insert(party_name, 0);
    }
    for r in &candidates {
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
    for r in &candidates {
        if r.sex == "男" {
            sex_ratio.men += r.vote_count;
        } else if r.sex == "女" {
            sex_ratio.women += r.vote_count;
        }
    }

    let data = IndexTmplContext {
        top_candidates: &candidates[0..10],
        worst_candidate: candidates.last().as_ref().unwrap(),
        parties,
        sex_ratio,
    };

    HttpResponse::Ok().body(data.render().unwrap())
}

#[derive(Template)]
#[template(path = "candidate.html")]
struct CandidateTmplContext {
    candidate: Candidate,
    votes: i32,
    keywords: Vec<String>,
}

async fn show_candidate(pool: web::Data<MySqlPool>, path: web::Path<(i32,)>) -> impl Responder {
    newrelic_transaction!("GET candidate");

    let id = path.0;
    let candidate = match get_candidate(&pool, id).await {
        Some(candidate) => candidate,
        None => return HttpResponse::Found().header(header::LOCATION, "/").finish(),
    };
    let votes = get_vote_count_by_candidate_id(&pool, id).await;
    let keywords = get_voice_of_supporter_of_candidate(&pool, id).await;

    let data = CandidateTmplContext {
        candidate,
        votes,
        keywords,
    };

    HttpResponse::Ok().body(data.render().unwrap())
}

#[derive(Template)]
#[template(path = "political_party.html")]
struct PoliticalPartyTmplContext {
    political_party: String,
    votes: i32,
    candidates: Vec<Candidate>,
    keywords: Vec<String>,
}

async fn show_political_party(
    pool: web::Data<MySqlPool>,
    path: web::Path<(String,)>,
) -> impl Responder {
    newrelic_transaction!("GET political_party");

    let political_party = &path.0;
    let votes = get_vote_count_by_political_party(&pool, political_party).await;

    let candidates = get_candidates_by_political_party(&pool, political_party).await;

    let keywords = get_voice_of_supporter_of_party(&pool, political_party).await;

    let data = PoliticalPartyTmplContext {
        political_party: political_party.clone(),
        votes,
        candidates,
        keywords,
    };

    HttpResponse::Ok().body(data.render().unwrap())
}

#[derive(Template)]
#[template(path = "vote.html")]
struct VoteTmplContext {
    candidates: Vec<Candidate>,
    message: String,
}

async fn show_vote(pool: web::Data<MySqlPool>) -> impl Responder {
    newrelic_transaction!("GET vote");

    let candidates = get_all_candidate(&pool).await;

    let data = VoteTmplContext {
        candidates,
        message: String::new(),
    };

    HttpResponse::Ok().body(data.render().unwrap())
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

async fn do_vote(pool: web::Data<MySqlPool>, form: web::Form<VoteFormData>) -> impl Responder {
    newrelic_transaction!("POST vote");

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
                let candidate = candidate.as_ref().unwrap();

                futures_join!(
                    create_vote(
                        &pool,
                        user.id,
                        candidate.id,
                        &form.keyword,
                        vote_count as i32,
                        &candidate.political_party,
                    ),
                    update_vote_count_of_candidate(&pool, candidate.id, vote_count as i32),
                    update_candidate_keyword(&pool, candidate.id, &form.keyword, vote_count as i32),
                    update_party_keyword(
                        &pool,
                        &candidate.political_party,
                        &form.keyword,
                        vote_count as i32,
                    )
                );
                "投票に成功しました"
            }
        }
    };

    let data = VoteTmplContext {
        candidates,
        message: message.to_owned(),
    };

    HttpResponse::Ok().body(data.render().unwrap())
}

async fn initialize(pool: web::Data<MySqlPool>) -> impl Responder {
    sqlx::query!("DELETE FROM votes")
        .execute(pool.get_ref())
        .await
        .expect("failed to initialize.");
    sqlx::query!("UPDATE candidates SET vote_count = 0;")
        .execute(pool.get_ref())
        .await
        .expect("failed to initialize candidates vote_count.");

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

    newrelic_init!();

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
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
