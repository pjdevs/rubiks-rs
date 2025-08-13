use crate::rubiks::{cube::{Cube, CubeMove}, faces::FaceMask, location::CubePieceLocation, solvers::pochmann::PochmannSolver, stickers::CubeStickerLocation, twist::Twist};

mod rubiks;

fn main() {
    use CubeMove::*;

    let mut cube = Cube::solved();
    cube.apply_moves(&vec![
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up
    ]);

    let solver = PochmannSolver {
        buffer_corner: CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UBL), twist:Twist::SOLVED },
        buffer_edge: CubeStickerLocation { piece_location: CubePieceLocation::new(FaceMask::UR), twist:Twist::SOLVED },
    };
    let solution = solver.solve(&cube);

    println!("C: {:?}", solution.corner_cycles);
}
