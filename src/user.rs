use sqlx::mysql::MySqlPool;

pub struct User {
    pub id: i32,
    pub name: String,
    pub address: String,
    pub mynumber: String,
    pub votes: i32,
}

pub async fn get_user(
    pool: &MySqlPool,
    name: &str,
    address: &str,
    my_number: &str,
) -> Option<User> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE name = ? AND address = ? AND mynumber = ?",
        name,
        address,
        my_number
    )
    .fetch_optional(pool)
    .await
    .expect("failed to fetch the user")
}
