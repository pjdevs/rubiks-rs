use rubiks::cube::{CubeState, CubeMove::*};
use rubiks::solvers::pochmann::PochmannSolver;
use rubiks::stickers::CubeStickerLocation;

fn main() {
    let mut cube = CubeState::solved();
    cube.apply_moves(&vec![R, U, Rp, Up, R, U, Rp, Up, R, U, Rp, Up]);

    let solver = PochmannSolver {
        buffer_corner: CubeStickerLocation::UBL,
        buffer_edge: CubeStickerLocation::UR,
    };
    let solution = solver.solve(&cube);

    println!("C: {:?}", solution.corner_cycles);
}
