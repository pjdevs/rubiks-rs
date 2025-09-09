use crate::services::scramble_service::ScrambleService;
use crate::services::daily_solves::DailySolvesService;
use crate::database::daily_solves::SqliteDailySolvesRepository;

#[derive(Clone)]
pub struct AppState {
    pub scramble_service: ScrambleService,
    pub daily_solve_service: DailySolvesService<SqliteDailySolvesRepository>
}