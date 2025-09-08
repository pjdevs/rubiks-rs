use crate::database::models::DailySolve;

#[async_trait::async_trait]
pub trait DailySolvesRepository: Send + Sync {
    async fn insert(&self, daily_solve: DailySolve) -> Result<bool, String>;
}