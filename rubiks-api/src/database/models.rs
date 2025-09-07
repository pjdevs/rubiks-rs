use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
struct DailySolve {
    id: i64,
    username: String,
    date: String,
    time: i64,
}