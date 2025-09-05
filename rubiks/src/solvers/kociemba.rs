// TODO Implement own Kociemba later

use kociemba::{facelet::Facelet, solver::solve};

use crate::cube::Cube;

pub struct KociembaSolver {

}

impl KociembaSolver {
    pub fn solve(&self, cube: &Cube) -> Option<String> {
        let mut rng = rand::rng();
        let cube = Cube::random_uniform(&mut rng);
        let string = "UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB";

        // solve(cubestring, max_length, timeout);

        None
    }
}