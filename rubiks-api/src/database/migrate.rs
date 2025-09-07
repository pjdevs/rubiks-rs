use sqlx::{Sqlite, SqlitePool};
use sqlx::migrate::MigrateDatabase;

const DB_URL: &str = "sqlite://rubiks.db";

pub async fn ensure_db() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL)
        .await
        .expect("Cannot connect to database for table creation.");
    
    let result = sqlx::query("CREATE TABLE IF NOT EXISTS daily_solves (id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, username VARCHAR(250) NOT NULL, date TEXT NOT NULL, time INTEGER NOT NULL);")
        .execute(&db)
        .await
        .expect("Could not create daily_solves table.");

        println!("Create daily_solves table result: {:?}", result);
}

#[tokio::main]
async fn main() {
    ensure_db().await;
}