use chrono::NaiveDate;
use crate::domain::models::DailySolve;
use crate::repositories::daily_solve::DailySolvesRepository;

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

    pub async fn add_daily_solve(&self, daily_solve: DailySolve) -> Result<(), String> {
        if daily_solve.time <= 0 {
            return Err("time is invalid.".into());
        }

        if daily_solve.username.is_empty() {
            return Err("username is empty.".into());
        }

        self.daily_repository.insert(daily_solve).await
    }

    pub async fn fetch_today_solves(&self, today: NaiveDate) -> Result<Vec<DailySolve>, String> {
        self.daily_repository.fetch_all_by_date(today).await
    }
}