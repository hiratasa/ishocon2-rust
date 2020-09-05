use serde::Serialize;

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
