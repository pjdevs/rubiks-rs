use rubiks::cube::{CubeState, CubeMove::*};
use rubiks::solvers::pochmann::PochmannSolver;
use rubiks::stickers::{CornerSticker, EdgeSticker};

fn main() {
    let mut cube = CubeState::solved();
    cube.apply_moves(&vec![R, U, Rp, Up, R, U, Rp, Up, R, U, Rp, Up]);

    let solver = PochmannSolver {
        buffer_corner: CornerSticker::UBL,
        buffer_edge: EdgeSticker::UR,
    };
    let solution = solver.solve(&cube);

    println!("C: {:?}", solution.corner_cycles);
}
