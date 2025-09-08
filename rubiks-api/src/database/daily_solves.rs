use sqlx::SqlitePool;

use crate::repositories::DailySolvesRepository;
use super::models::DailySolve;

#[derive(Clone)]
pub struct SqliteDailySolvesRepository {
    pool: SqlitePool
}

impl SqliteDailySolvesRepository {
    pub fn new(pool: SqlitePool) -> Self{
        Self {
            pool
        }
    }
}

#[async_trait::async_trait]
impl DailySolvesRepository for SqliteDailySolvesRepository {
    async fn insert(&self, daily_solve: DailySolve) -> Result<bool, String> {
        let result = sqlx::query("INSERT INTO daily_solves (username, date, time) VALUES (?1, ?2, ?3);")
            .bind(daily_solve.username)
            .bind(daily_solve.date)
            .bind(daily_solve.time)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(true),
            Err(err) => {
                let db_error = err.as_database_error().expect("Error should be db error.");

                if db_error.constraint().is_some() {
                    Ok(false)
                } else {
                    Err(db_error.message().to_string())
                }
            },
        }
    }
}

    