use crate::rubiks::cube::{Cube, CubeMove};

mod rubiks;

fn main() {
    use CubeMove::*;

    let mut cube = Cube::solved();
    cube.apply_moves(&vec![
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up,
    ]);
    assert_eq!(cube, Cube::solved());

    // let solver = PochmannSolver {
    //     buffer_corner: UBL,
    //     buffer_edge: UR
    // };
    // let solution = solver.solve(&cube);

    // println!("C: {:?}", solution.corner_cycles);
    // println!("E: {:?}", solution.corner_cycles);
}
