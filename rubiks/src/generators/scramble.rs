use rand::Rng;

use crate::solvers::kociemba::KociembaSolver;
use crate::cube::{Cube, CubeMove};

#[derive(Clone, Copy)]
pub struct ScrambleGenerator {
    solver: KociembaSolver
}

impl Default for ScrambleGenerator {
    fn default() -> Self {
        Self {
            solver: KociembaSolver::default()
        }
    }
}

impl ScrambleGenerator {
    pub fn generate(&self, rng: &mut impl Rng) -> Vec<CubeMove> {
        let cube = Cube::random_uniform(rng);
        self.solver.solve(&cube).expect("Random uniform cube should have a solution")
    }
}