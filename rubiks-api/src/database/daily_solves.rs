use chrono::{Datelike, NaiveDate};
use sqlx::SqlitePool;
use crate::{domain::models::DailySolve, repositories::daily_solve::DailySolvesRepository};
use super::rows::DailySolveRow;


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
    async fn insert(&self, daily_solve: DailySolve) -> Result<(), String> {
        let result = sqlx::query("INSERT INTO daily_solves (username, date, time) VALUES (?1, ?2, ?3);")
            .bind(daily_solve.username)
            .bind(daily_solve.date.num_days_from_ce())
            .bind(daily_solve.time)
            .execute(&self.pool)
            .await;

        result
            .map(|_| ())
            .map_err(db_error_to_string)
    }

    async fn fetch_all_by_date(&self, date: NaiveDate) -> Result<Vec<DailySolve>, String> {
        let result = sqlx::query_as::<_, DailySolveRow>("SELECT username, date, time FROM daily_solves WHERE date = ?1 ORDER BY time ASC;")
            .bind(date.num_days_from_ce())
            .fetch_all(&self.pool)
            .await;

        result
            .map(|solves| solves.into_iter().map(DailySolveRow::into).collect())
            .map_err(|error| db_error_to_string(error))
    }
}

fn db_error_to_string(error: sqlx::Error) -> String {
     error
            .as_database_error()
            .expect("Error should be a database error.")
            .message()
            .to_string()
}

impl Into<DailySolve> for DailySolveRow {
    fn into(self) -> DailySolve {
        DailySolve {
            username: self.username,
            date: NaiveDate::from_num_days_from_ce_opt(self.date).expect("Db date should be valid"),
            time: self.time
        }
    }
}