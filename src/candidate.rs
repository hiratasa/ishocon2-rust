use futures::TryStreamExt;
use serde::Serialize;
use sqlx::mysql::MySqlPool;

#[derive(Serialize)]
pub struct Candidate {
    pub id: i32,
    pub name: String,
    pub political_party: String,
    pub sex: String,
}

#[derive(Serialize)]
pub struct CandidateElectionResult {
    pub id: i32,
    pub name: String,
    pub political_party: String,
    pub sex: String,
    pub vote_count: i32,
}

#[derive(Serialize)]
pub struct PartyElectionResult {
    pub political_party: String,
    pub vote_count: i32,
}

pub async fn get_all_candidate(pool: &MySqlPool) -> Vec<Candidate> {
    sqlx::query_as!(Candidate, "SELECT * FROM candidates")
        .fetch_all(pool)
        .await
        .expect("failed to fetch all candidates")
}

pub async fn get_candidate(pool: &MySqlPool, id: i32) -> Option<Candidate> {
    sqlx::query_as!(Candidate, "SELECT * FROM candidates WHERE id = ?", id)
        .fetch_optional(pool)
        .await
        .expect("failed to fetch the candidate")
}

pub async fn get_candidate_by_name(pool: &MySqlPool, name: &str) -> Option<Candidate> {
    sqlx::query_as!(Candidate, "SELECT * FROM candidates WHERE name = ?", name)
        .fetch_optional(pool)
        .await
        .expect("failed to fetch the candidate")
}

pub async fn get_all_party_name(pool: &MySqlPool) -> Vec<String> {
    sqlx::query!("SELECT political_party FROM candidates GROUP BY political_party")
        .fetch(pool)
        .map_ok(|row| row.political_party)
        .try_collect()
        .await
        .expect("failed to fetch all party names")
}

pub async fn get_candidates_by_political_party(pool: &MySqlPool, party: &str) -> Vec<Candidate> {
    sqlx::query_as!(
        Candidate,
        "SELECT * FROM candidates WHERE political_party = ?",
        party
    )
    .fetch_all(pool)
    .await
    .expect("failed to fetch candidate by party")
}
