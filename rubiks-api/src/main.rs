use axum::{Json, Router};
use axum::http::StatusCode;
use axum::extract::Query;
use axum::routing::get;
use rand::seq::IndexedRandom;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use std::collections::HashMap;
use rubiks::cube::{Cube, CubeMove};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/daily/scramble", get(get_daily_scramble))
        .route("/cube/solved", get(get_cube_is_solved))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

static SCRAMBLES: [&str; 3] = [
    "B2 D2 F' D2 R' D B2 U' R F' R2 F' U2 R2 F' R2 B2 U2 R2 B",
    "B D R2 F U' F' U R F2 R2 B2 R' F2 L' U2 L2 F2 B",
    "D' B D R2 F2 D2 R F2 R' B2 L' D2 R2 D2 F U2 R B' U L F2",
];

async fn get_daily_scramble() -> &'static str {
    SCRAMBLES.choose(&mut rand::rng()).unwrap_or(&SCRAMBLES[0])
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