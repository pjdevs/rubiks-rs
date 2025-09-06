use axum::{Json, Router};
use axum::http::StatusCode;
use axum::extract::{Query, State};
use axum::routing::get;
use rubiks::generators::scramble::ScrambleGenerator;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use std::collections::HashMap;
use rubiks::cube::{Cube, CubeMove};

#[derive(Clone, Default)]
struct AppState {
    generator: ScrambleGenerator
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
        .with_state(AppState::default());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_scramble(State(state): State<AppState>) -> String {
    let scramble = state.generator.generate();
    scramble.iter().map(|m| m.to_string()).collect::<Vec<String>>().join(" ")
}

// TODO Make it daily
async fn get_daily_scramble(State(state): State<AppState>) -> String {
    let scramble = state.generator.generate();
    scramble.iter().map(|m| m.to_string()).collect::<Vec<String>>().join(" ")
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
