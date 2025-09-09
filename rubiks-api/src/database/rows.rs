use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct DailySolveRow {
    pub username: String,
    pub date: i32,
    pub time: i32,
}