use serde::{Deserialize, Serialize};

use crate::domain::models::DailySolve;

#[derive(Deserialize)]
pub struct DailySolveRequestDto {
    pub username: String,
    pub time: i32,
}

#[derive(Serialize)]
pub struct DailySolveDto {
    pub username: String,
    pub time: i32,
}

#[derive(Serialize)]
pub struct DailySolveListDto {
    pub solves: Vec<DailySolveDto>
}

impl From<DailySolve> for DailySolveDto {
    fn from(solve: DailySolve) -> Self {
        Self {
            username: solve.username,
            time: solve.time,
        }
    }
}

impl From<Vec<DailySolve>> for DailySolveListDto {
    fn from(solves: Vec<DailySolve>) -> Self {
        Self {
            solves: solves.into_iter().map(DailySolveDto::from).collect(),
        }
    }
}
