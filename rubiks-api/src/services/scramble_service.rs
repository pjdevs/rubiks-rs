use std::sync::Arc;
use chrono::{Datelike, NaiveDate};
use rand::{rngs::StdRng, SeedableRng};
use rubiks::{cube::{Cube, CubeMove}, generators::scramble::ScrambleGenerator};

#[derive(Clone)]
pub struct ScrambleService {
    generator: Arc<ScrambleGenerator>
}

impl ScrambleService {
    pub fn new() -> Self {
        Self {
            generator: Arc::new(ScrambleGenerator::default()),
        }
    }

    pub async fn get_daily_scramble(&self, date: NaiveDate) -> String {
        let seed = (date.year() as u64) * 10_000u64 + (date.month() as u64) * 100u64 + date.day() as u64;
        self.generate_scramble_with_seed(Some(seed)).await
    }

    pub async fn get_random_scramble(&self) -> String {
        self.generate_scramble_with_seed(None).await
    }

    pub fn is_cube_solved(&self, moves_str: &str) -> Option<bool> {   
        CubeMove::parse_array(&moves_str)
            .map(|moves| {
                let mut cube = Cube::solved();
                cube.apply_moves(&moves);
                cube.is_solved()
            })
    }

    async fn generate_scramble_with_seed(&self, seed: Option<u64>) -> String {
        let generator = self.generator.clone();
        let scramble = tokio::task::spawn_blocking(move || {
            let scramble_moves = match seed {
                Some(seed) => {
                    let mut rng = StdRng::seed_from_u64(seed);
                    generator.generate(&mut rng)
                }
                None => {
                    let mut rng = rand::rng();
                    generator.generate(&mut rng)
                },
            };
            scramble_moves.iter().map(|m| m.to_string()).collect::<Vec<String>>().join(" ")
        })
        .await
        .expect("Scramble generation panicked.");

        scramble
    }
}