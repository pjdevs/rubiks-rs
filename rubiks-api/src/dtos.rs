use serde::Deserialize;

#[derive(Deserialize)]
pub struct DailySolveRequest {
    pub username: String,
    pub time: i32,
}