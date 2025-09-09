use rubiks_api::api::state::AppState;
use rubiks_api::api::routes::build_router;
use rubiks_api::database::pool::build_pool;
use rubiks_api::database::daily_solves::SqliteDailySolvesRepository;
use rubiks_api::services::daily_solves::DailySolvesService;
use rubiks_api::services::scramble_service::ScrambleService;

#[tokio::main]
async fn main() {
    // create the connection pool
    let pool = build_pool();

    // build application
    let app_state = AppState {
        scramble_service: ScrambleService::new(),
        daily_solve_service: DailySolvesService::new(SqliteDailySolvesRepository::new(pool))
    };
    let app = build_router(app_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
