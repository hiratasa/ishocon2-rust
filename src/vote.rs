use futures::TryStreamExt;
use sqlx::mysql::MySqlPool;

pub async fn get_user_voted_count(pool: &MySqlPool, user_id: i32) -> i64 {
    sqlx::query!(
        "SELECT IFNULL(CAST(SUM(vote_count) AS SIGNED), 0) AS vote_count FROM votes WHERE user_id = ?",
        user_id
    )
    .fetch_one(pool)
    .await
    .expect("failed to fetch vote count by user id")
    .vote_count
}

pub async fn create_vote(
    pool: &MySqlPool,
    user_id: i32,
    candidate_id: i32,
    keyword: &str,
    vote_count: i32,
    political_party: &str,
) {
    sqlx::query!(
        "INSERT INTO votes (user_id, candidate_id, keyword, vote_count, political_party) VALUES (?, ?, ?, ?, ?)",
        user_id,
        candidate_id,
        keyword,
        vote_count,
        political_party
    )
    .execute(pool)
    .await
    .expect("failed to create vote");
}

pub async fn update_candidate_keyword(
    pool: &MySqlPool,
    candidate_id: i32,
    keyword: &str,
    vote_count: i32,
) {
    sqlx::query!(
        "INSERT INTO keywords (keyword, candidate_id, vote_count) VALUES (?, ?, ?)
        ON DUPLICATE KEY UPDATE vote_count = vote_count + ?",
        keyword,
        candidate_id,
        vote_count,
        vote_count,
    )
    .execute(pool)
    .await
    .expect("failed to update keyword of candidate");
}

pub async fn update_party_keyword(
    pool: &MySqlPool,
    political_party: &str,
    keyword: &str,
    vote_count: i32,
) {
    sqlx::query!(
        "INSERT INTO party_keywords (keyword, political_party, vote_count) VALUES (?, ?, ?)
        ON DUPLICATE KEY UPDATE vote_count = vote_count + ?",
        keyword,
        political_party,
        vote_count,
        vote_count,
    )
    .execute(pool)
    .await
    .expect("failed to update keyword of party");
}

pub async fn get_voice_of_supporter_of_candidate(
    pool: &MySqlPool,
    candidate_id: i32,
) -> Vec<String> {
    sqlx::query!(
        "SELECT keyword FROM keywords WHERE candidate_id = ? ORDER BY vote_count DESC LIMIT 10",
        candidate_id
    )
    .fetch(pool)
    .map_ok(|row| row.keyword)
    .try_collect()
    .await
    .expect("failed to get voice of supporters of candidate")
}

pub async fn get_voice_of_supporter_of_party(
    pool: &MySqlPool,
    political_party: &str,
) -> Vec<String> {
    sqlx::query!("SELECT keyword FROM party_keywords WHERE political_party = ? ORDER BY vote_count DESC LIMIT 10", political_party)
        .fetch(pool)
        .map_ok(|row| row.keyword)
        .try_collect()
        .await
        .expect("failed to get voice of supporters of political party")
}
