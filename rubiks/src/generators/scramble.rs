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
    pub fn generate(&self) -> Vec<CubeMove> {
        let mut rng = rand::rng();
        let cube = Cube::random_uniform(&mut rng);
        self.solver.solve(&cube).expect("Random uniform cube should have a solution")
    }
}