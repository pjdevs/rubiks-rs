use sqlx::SqlitePool;

use crate::database::constants::DB_URL;

pub fn build_pool() -> SqlitePool {
    SqlitePool::connect_lazy(DB_URL).expect("Cannot create lazy pool to SQLite database.")
}