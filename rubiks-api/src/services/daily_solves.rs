use chrono::{Datelike, NaiveDate};

use crate::database::models::DailySolve;
use crate::dtos::DailySolveRequest;
use crate::repositories::DailySolvesRepository;

#[derive(Clone, Copy)]
pub struct DailySolvesService<R: DailySolvesRepository> {
    daily_repository: R
}

impl<R: DailySolvesRepository> DailySolvesService<R> {
    pub fn new(daily_repository: R) -> Self {
        Self {
            daily_repository,
        }
    }

    pub async fn add_daily_solve(&self, request: DailySolveRequest, today: NaiveDate) -> Result<bool, String> {
        if request.time <= 0 {
            return Err("time is invalid.".into());
        }

        let daily_solve = DailySolve {
            username: request.username,
            date: today.num_days_from_ce(),
            time: request.time
        };
        self.daily_repository.insert(daily_solve).await
    }
}