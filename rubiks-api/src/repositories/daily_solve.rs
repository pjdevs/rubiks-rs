use chrono::NaiveDate;
use crate::domain::models::DailySolve;

#[async_trait::async_trait]
pub trait DailySolvesRepository: Send + Sync {
    async fn insert(&self, daily_solve: DailySolve) -> Result<(), String>;
    async fn fetch_all_by_date(&self, date: NaiveDate) -> Result<Vec<DailySolve>, String>;
}