use crate::rubiks::solvers::pochmann::PochmannSolver;
use crate::rubiks::stickers::CubeStickerLocation;
use crate::rubiks::cube::{Cube, CubeMove::*};

mod rubiks;

fn main() {
    let mut cube = Cube::solved();
    cube.apply_moves(&vec![
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up
    ]);

    let solver = PochmannSolver {
        buffer_corner: CubeStickerLocation::UBL,
        buffer_edge: CubeStickerLocation::UR,
    };
    let solution = solver.solve(&cube);

    println!("C: {:?}", solution.corner_cycles);
}
