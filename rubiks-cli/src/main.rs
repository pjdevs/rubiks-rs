use rubiks::cube::{Cube, CubeMove::*};
use rubiks::solvers::pochmann::PochmannSolver;
use rubiks::stickers::CubeStickerLocation;

fn main() {
    let mut cube = Cube::solved();
    cube.apply_moves(&vec![R, U, Rp, Up]);

    // let solver = PochmannSolver {
    //     buffer_corner: CubeStickerLocation::UBL,
    //     buffer_edge: CubeStickerLocation::UR,
    // };
    // let solution = solver.solve(&cube);

    // println!("C: {:?}", solution.corner_cycles);
    // println!("E: {:?}", solution.edge_cycles);

    println!("{:?}", cube.get_sticker_origin(&CubeStickerLocation::UB).to_sticker_name());
    println!("{:?}", cube.get_sticker_origin(&CubeStickerLocation::BU).to_sticker_name());

    println!();

    println!("{:?}", cube.get_sticker_origin(&CubeStickerLocation::UR).to_sticker_name());
    println!("{:?}", cube.get_sticker_origin(&CubeStickerLocation::RU).to_sticker_name());

    println!();

    println!("{:?}", cube.get_sticker_origin(&CubeStickerLocation::FR).to_sticker_name());
    println!("{:?}", cube.get_sticker_origin(&CubeStickerLocation::RF).to_sticker_name());

    println!();

    println!("{:?}", cube.get_sticker_origin(&CubeStickerLocation::UBR).to_sticker_name());
    println!("{:?}", cube.get_sticker_origin(&CubeStickerLocation::BRU).to_sticker_name());
    println!("{:?}", cube.get_sticker_origin(&CubeStickerLocation::RUB).to_sticker_name());

    println!();

    println!("{:?}", cube.get_sticker_origin(&CubeStickerLocation::URF).to_sticker_name());
    println!("{:?}", cube.get_sticker_origin(&CubeStickerLocation::RFU).to_sticker_name());
    println!("{:?}", cube.get_sticker_origin(&CubeStickerLocation::FUR).to_sticker_name());
}
