use axum::{extract, Json, Router};
use axum::http::StatusCode;
use axum::extract::{Query, State};
use axum::routing::{get, post};
use chrono::Utc;
use sqlx::SqlitePool;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use std::collections::HashMap;
use crate::database::daily_solves::SqliteDailySolvesRepository;
use crate::dtos::DailySolveRequest;
use crate::services::daily_solves::DailySolvesService;
use crate::services::scramble_service::ScrambleService;

mod database;
mod repositories;
mod services;
mod dtos;

#[derive(Clone)]
pub struct AppState {
    scramble_service: ScrambleService,
    daily_solve_service: DailySolvesService<SqliteDailySolvesRepository>
}

#[tokio::main]
async fn main() {
    // ensure db is created
    database::migrate::ensure_db().await;

    // create the connection pool
    let pool = SqlitePool::connect_lazy(database::constants::DB_URL)
        .expect("Cannot create lazy pool to SQLite database.");

    // build our application with a single route
    let app = Router::new()
        .route("/cube/scramble", get(get_cube_scramble))
        .route("/cube/solved", get(get_cube_is_solved))
        .route("/daily/scramble", get(get_daily_scramble))
        .route("/daily/scramble", post(post_daily_scramble))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(AppState {
            scramble_service: ScrambleService::new(),
            daily_solve_service: DailySolvesService::new(SqliteDailySolvesRepository::new(pool))
        });

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_cube_scramble(State(state): State<AppState>) -> String {
    state.scramble_service.get_random_scramble().await
}

async fn get_daily_scramble(State(state): State<AppState>) -> String {
    let date = Utc::now().date_naive();
    state.scramble_service.get_daily_scramble(date).await
}

async fn post_daily_scramble(
    State(state): State<AppState>,
    extract::Json(request): extract::Json<DailySolveRequest>
) -> Result<(), (StatusCode, String)> {
    let result = state.daily_solve_service.add_daily_solve(request, Utc::now().date_naive()).await;
    
    match result {
        Ok(ok) => if ok {
            Ok(())
        } else {
            Err((StatusCode::BAD_REQUEST, "Daily time already submited.".into()))
        },
        Err(err_str) => Err((StatusCode::BAD_REQUEST, err_str)),
    }
}

// TODO Make an axum extractor for scrambles
pub async fn get_cube_is_solved(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>
) -> Result<Json<bool>, (StatusCode, String)> {
    let moves_key = params.get("moves");

    match moves_key {
        Some(moves_str) => {
            match state.scramble_service.is_cube_solved(moves_str) {
                Some(is_solved) => Ok(Json(is_solved)),
                None => Err((StatusCode::BAD_REQUEST, "'moves' sequence is invalid.".into())),
            }
        },
        None => Err((StatusCode::BAD_REQUEST, "'moves' query parameter is missing.".into())),
    }
}
