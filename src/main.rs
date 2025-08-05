use cube::{Cube, CubeMove};

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

    println!("{:?}", cube == Cube::solved());
}
