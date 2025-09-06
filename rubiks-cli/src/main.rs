use rubiks::cube::{Cube, CubeMove::*};
use rubiks::solvers::pochmann::PochmannSolver;
use rubiks::stickers::CubeStickerLocation;

fn main() {
    let mut cube = Cube::solved();
    cube.apply_moves(&vec![R, U, Rp, Up]);

    let solver = PochmannSolver {
        buffer_corner: CubeStickerLocation::ULB,
        buffer_edge: CubeStickerLocation::UR,
    };
    let solution = solver.solve(&cube);

    println!("C: {:?}", solution.corner_cycles);
    println!("E: {:?}", solution.edge_cycles);
}
