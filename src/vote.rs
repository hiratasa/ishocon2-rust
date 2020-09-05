use sqlx::mysql::{MySqlPool, MySqlRow};
use sqlx::Row;

struct Vote {
    id: i32,
    user_id: i32,
    candidate_id: i32,
    keyword: String,
}

pub async fn get_vote_count_by_candidate_id(pool: &MySqlPool, candidate_id: i32) -> i64 {
    sqlx::query!(
        "SELECT COUNT(*) AS vote_count FROM votes WHERE candidate_id = ?",
        candidate_id
    )
    .fetch_one(pool)
    .await
    .expect("failed to fetch vote count by candidate id")
    .vote_count
}

pub async fn get_user_voted_count(pool: &MySqlPool, user_id: i32) -> i64 {
    sqlx::query!(
        "SELECT COUNT(*) AS vote_count FROM votes WHERE user_id = ?",
        user_id
    )
    .fetch_one(pool)
    .await
    .expect("failed to fetch vote count by user id")
    .vote_count
}

pub async fn create_vote(pool: &MySqlPool, user_id: i32, candidate_id: i32, keyword: &str) {
    sqlx::query!(
        "INSERT INTO votes (user_id, candidate_id, keyword) VALUES (?, ?, ?)",
        user_id,
        candidate_id,
        keyword
    )
    .execute(pool)
    .await
    .expect("failed to create vote");
}

pub async fn get_voice_of_supporter(pool: &MySqlPool, candidate_ids: &Vec<i32>) -> Vec<String> {
    // Not use macro query!, because the number of ids are dynamic.
    sqlx::query(
        &(String::from(
            "
            SELECT keyword
            FROM votes
            WHERE candidate_id IN (",
        ) + &vec!["?"; candidate_ids.len()].join(",")
            + ")
            GROUP BY keyword
            ORDER BY COUNT(*) DESC
            LIMIT 10"),
    )
    .try_map(|row: MySqlRow| row.try_get(0))
    .fetch_all(pool)
    .await
    .expect("failed to get voice of supporters")
}
