// TODO Implement own Kociemba later

use crate::{cube::{Cube, CubeMove}, faces::Face, stickers::CubeStickerLocation};
use kociemba::{moves::Move, solver::solve};

pub struct KociembaSolver {
    pub max_size: usize,
    pub timeout: f32,
}

impl Default for KociembaSolver {
    fn default() -> Self {
        Self { max_size: 20, timeout: 0.0 }
    }
}

impl KociembaSolver {
    pub fn solve(&self, cube: &Cube) -> Option<Vec<CubeMove>> {
        let cube_string = make_cube_string(&cube);

        println!("Will solve cube : {}", cube_string);

        match solve(&cube_string, 20, 1.0) {
            Ok(solution) => Some(
                solution.solution
                .into_iter()
                .map(|m| m.into())
                .collect()
            ),
            Err(error) => {
                println!("{:?}", error);
                None
            },
        }
    }
}

impl Into<CubeMove> for Move {
    fn into(self) -> CubeMove {
        match self {
            Move::U => CubeMove::U,
            Move::U2 => CubeMove::U2,
            Move::U3 => CubeMove::Up,
            Move::R => CubeMove::R,
            Move::R2 => CubeMove::R2,
            Move::R3 => CubeMove::Rp,
            Move::F => CubeMove::F,
            Move::F2 => CubeMove::F2,
            Move::F3 => CubeMove::Fp,
            Move::D => CubeMove::D,
            Move::D2 => CubeMove::D2,
            Move::D3 => CubeMove::Dp,
            Move::L => CubeMove::L,
            Move::L2 => CubeMove::L2,
            Move::L3 => CubeMove::Lp,
            Move::B => CubeMove::B,
            Move::B2 => CubeMove::B2,
            Move::B3 => CubeMove::Bp,
        }
    }
}

/// Make a string representation of the cube in the form
/// "UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB".
fn make_cube_string(cube: &Cube) -> String {
    let mut cube_string = String::with_capacity(54);
    let mut start = 0;

    // On each face
    for face in FACES {
        // For each stickers of the face
        for i in start..(start + 8) {            
            // Add sticker to the cube string
            let sticker_location = FACELET_POSITIONS[i];
            let sticker_name = cube.get_sticker_origin(&sticker_location).to_sticker_name();
            let sticker_face = sticker_name.chars().nth(0).expect("Stickers name have a least one character.");
            cube_string.push(sticker_face);

            // An add also center which we don't have in our impl
            if i == start + 3 {
                cube_string.push_str(&face.to_string());
            }
        }

        start += 8;
    }

    cube_string
}

static FACELET_POSITIONS: [CubeStickerLocation; 48]  = [
    // U
    CubeStickerLocation::ULB,
    CubeStickerLocation::UB,
    CubeStickerLocation::UBR,
    CubeStickerLocation::UL,
    CubeStickerLocation::UR,
    CubeStickerLocation::UFL,
    CubeStickerLocation::UF,
    CubeStickerLocation::URF,

    // R
    CubeStickerLocation::RFU,
    CubeStickerLocation::RU,
    CubeStickerLocation::RUB,
    CubeStickerLocation::RF,
    CubeStickerLocation::RB,
    CubeStickerLocation::RDF,
    CubeStickerLocation::RD,
    CubeStickerLocation::RBD,

    // F
    CubeStickerLocation::FLU,
    CubeStickerLocation::FU,
    CubeStickerLocation::FUR,
    CubeStickerLocation::FL,
    CubeStickerLocation::FR,
    CubeStickerLocation::FDL,
    CubeStickerLocation::FD,
    CubeStickerLocation::FRD,

    // D
    CubeStickerLocation::DLF,
    CubeStickerLocation::DF,
    CubeStickerLocation::DFR,
    CubeStickerLocation::DL,
    CubeStickerLocation::DR,
    CubeStickerLocation::DBL,
    CubeStickerLocation::DB,
    CubeStickerLocation::DRB,

    // L
    CubeStickerLocation::LBU,
    CubeStickerLocation::LU,
    CubeStickerLocation::LUF,
    CubeStickerLocation::LB,
    CubeStickerLocation::LF,
    CubeStickerLocation::LDB,
    CubeStickerLocation::LD,
    CubeStickerLocation::LFD,

    // B
    CubeStickerLocation::BRU,
    CubeStickerLocation::BU,
    CubeStickerLocation::BUL,
    CubeStickerLocation::BR,
    CubeStickerLocation::BL,
    CubeStickerLocation::BDR,
    CubeStickerLocation::BD,
    CubeStickerLocation::BLD,
];

static FACES: [Face; 6] = [
    Face::U,
    Face::R,
    Face::F,
    Face::D,
    Face::L,
    Face::B,
];

#[cfg(test)]
mod tests {
    use crate::location::CubePieceLocation;
    use crate::solvers::kociemba::{make_cube_string, KociembaSolver};
    use crate::cube::{Cube, CubeMove};

    #[test]
    fn test_solution_solve_cube() {
        let mut rng = rand::rng();
        let solver = KociembaSolver::default(); 

        for _ in 0..2 {
            let mut cube = Cube::random_uniform(&mut rng);
            let solution = solver.solve(&cube).expect("Random uniform cube should be solvable.");

            cube.apply_moves(&solution);
            assert!(cube.is_solved());
        }
    }

    #[test]
    fn test_cube_string_face_count() {
        let mut rng = rand::rng();

        for _ in 0..1000 {
            let cube = Cube::random_uniform(&mut rng);
            let cube_string = make_cube_string(&cube);
    
            assert_eq!(cube_string.len(), 54, "Cube string should be of size 54.");            
            assert_eq!(cube_string.chars().filter(|c| *c == 'U').count(), 9, "Should be 9 Us in cube string.");
            assert_eq!(cube_string.chars().filter(|c| *c == 'R').count(), 9, "Should be 9 Rs in cube string.");
            assert_eq!(cube_string.chars().filter(|c| *c == 'F').count(), 9, "Should be 9 Fs in cube string.");
            assert_eq!(cube_string.chars().filter(|c| *c == 'D').count(), 9, "Should be 9 Ds in cube string.");
            assert_eq!(cube_string.chars().filter(|c| *c == 'B').count(), 9, "Should be 9 Bs in cube string.");
            assert_eq!(cube_string.chars().filter(|c| *c == 'L').count(), 9, "Should be 9 Ls in cube string.");            
        }
    }

    #[test]
    fn test_cube_string_expected_solved() {
        let cube = Cube::solved();
        let cube_string = make_cube_string(&cube);
        assert_eq!(cube_string, "UUUUUUUUURRRRRRRRRFFFFFFFFFDDDDDDDDDLLLLLLLLLBBBBBBBBB");
    }

    #[test]
    fn test_cube_string_expected_l() {
        let mut cube = Cube::solved();
        let scramble = CubeMove::parse_array("L").expect("Scramble is valid.");
        cube.apply_moves(&scramble);

        let cube_string = make_cube_string(&cube);
        assert_eq!(cube_string, "BUUBUUBUURRRRRRRRRUFFUFFUFFFDDFDDFDDLLLLLLLLLBBDBBDBBD");
    }


    #[test]
    fn test_cube_string_expected_t_perm() {
        let mut cube = Cube::solved();
        let scramble = CubeMove::parse_array("R U R' U' R' F R2 U' R' U' R U R' F'").expect("Scramble is valid.");
        cube.apply_moves(&scramble);

        let cube_string = make_cube_string(&cube);
        assert_eq!(cube_string, "UUUUUUUUUBLFRRRRRRFFRFFFFFFDDDDDDDDDLRLLLLLLLRBBBBBBBB");
    }

    #[test]
    fn test_cube_string_expected_damier() {
        let mut cube = Cube::solved();
        let scramble = CubeMove::parse_array("R2 L2 B2 F2 U2 D2")
            .expect("Scramble is valid.");
        cube.apply_moves(&scramble);

        let cube_string = make_cube_string(&cube);
        assert_eq!(cube_string, "UDUDUDUDURLRLRLRLRFBFBFBFBFDUDUDUDUDLRLRLRLRLBFBFBFBFB");
    }

    #[test]
    fn test_cube_string_expected_scramble() {
        let mut cube = Cube::solved();
        let scramble = CubeMove::parse_array("B2 U2 R F' L2 U' L F' D2 R F2 L' B2 D2 B2 R2 U2 L U2 B2").expect("Scramble is valid.");
        cube.apply_moves(&scramble);

        println!("{:?}", cube.get_piece_at(&CubePieceLocation::ULB).get_original_location().get_faces());

        let cube_string = make_cube_string(&cube);
        assert_eq!(cube_string, "FFBBUBFUDFDRDRLULDUBRUFRFRRDFBBDFLDLLRRDLRULLDUUUBLBFB");
    }
}
