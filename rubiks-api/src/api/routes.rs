use std::collections::HashMap;
use axum::extract::{self, Query, State};
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use chrono::Utc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use crate::api::dtos::{DailySolveDto, DailySolveListDto, DailySolveRequestDto};
use crate::api::state::AppState;
use crate::domain::models::DailySolve;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/cube/scramble", get(get_cube_scramble))
        .route("/cube/solved", get(get_cube_is_solved))
        .route("/daily/scramble", get(get_daily_scramble))
        .route("/daily/scramble", post(post_daily_scramble))
        .route("/daily/leaderboard", get(get_daily_leaderboard))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
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
    extract::Json(request): extract::Json<DailySolveRequestDto>
) -> Result<(), (StatusCode, String)> {
    let solve = DailySolve {
        username: request.username,
        date: Utc::now().date_naive(),
        time: request.time,
    };
    let result = state.daily_solve_service.add_daily_solve(solve).await;
    
    result.map_err(|err_str| (StatusCode::BAD_REQUEST, err_str))
}

async fn get_daily_leaderboard(State(state): State<AppState>) -> Result<Json<DailySolveListDto>, (StatusCode, String)> {
    let date = Utc::now().date_naive();
    let solves = state.daily_solve_service.fetch_today_solves(date).await;

    match solves {
        Ok(solves) => Ok(Json(DailySolveListDto::from(solves))),
        Err(err_str) => Err((StatusCode::BAD_REQUEST, err_str)),
    }

}

// TODO Make an axum extractor for scrambles / put logic into service
// Was here to test at start but useless now 
async fn get_cube_is_solved(
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
