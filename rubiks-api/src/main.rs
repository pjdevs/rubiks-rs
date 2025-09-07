use axum::{Json, Router};
use axum::http::StatusCode;
use axum::extract::{Query, State};
use axum::routing::get;
use date::Date;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use std::collections::HashMap;
use std::sync::Arc;

use crate::services::scramble_service::ScrambleService;

mod services;

#[derive(Clone)]
pub struct AppState {
    scramble_service: ScrambleService
}

impl AppState {
    pub fn new() -> Self {
        Self { scramble_service: ScrambleService::new() }
    }
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/scramble", get(get_scramble))
        .route("/daily/scramble", get(get_daily_scramble))
        .route("/cube/solved", get(get_cube_is_solved))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(AppState::new()));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// TODO Make it daily
async fn get_daily_scramble(State(state): State<Arc<AppState>>) -> String {
    let date = Date::today_utc();
    state.scramble_service.get_daily_scramble(date).await
}

async fn get_scramble(State(state): State<Arc<AppState>>) -> String {
    generate_scramble(state).await
}

// TODO Make a scramble extractor
async fn get_cube_is_solved(Query(params): Query<HashMap<String, String>>) -> Result<Json<bool>, (StatusCode, String)> {
    let moves_key = params.get("moves");

    match moves_key {
        Some(moves_str) => {
            match CubeMove::parse_array(&moves_str) {
                Some(moves) => {
                    let mut cube = Cube::solved();
                    cube.apply_moves(&moves);

                    Ok(Json(cube.is_solved()))
                },
                None => Err((StatusCode::BAD_REQUEST, "'moves' is invalid.".into())),
            }
        },
        None => Err((StatusCode::BAD_REQUEST, "'moves' query parameter is missing.".into())),
    }
}
