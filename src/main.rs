use cube::{Cube, CubeMove, CornerSticker::*, EdgeSticker::*};

use crate::pochmann::PochmannSolver;

mod cube;
mod pochmann;

fn main() {
    use CubeMove::*;

    let mut cube = Cube::solved();
    cube.apply_moves(&vec![
        R, R, R, R,
        U, U, U, U,
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up,
    ]);
    assert_eq!(cube, Cube::solved());

    let solver = PochmannSolver
    {
        buffer_corner: UBL,
        buffer_edge: UR
    };
    let solution = solver.solve(&cube);

    println!("C: {:?}", solution.corner_cycles);
    println!("E: {:?}", solution.corner_cycles);
}
