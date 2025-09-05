use rand::Rng;

use crate::location::CubePieceLocation;
use crate::piece::CubePiece;
use crate::stickers::CubeStickerLocation;
use crate::twist::Twist;
use crate::utils::permutations;
use std::collections::HashMap;
use std::iter::once;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeMove {
    U,
    R,
    Up,
    Rp,
}

impl CubeMove {
    pub fn inverted(&self) -> Self {
        match self {
            CubeMove::U => Self::Up,
            CubeMove::R => Self::Rp,
            CubeMove::Up => Self::U,
            CubeMove::Rp => Self::R,
        }
    }

    pub fn parse(str: &str) -> Option<Self> {
        match str {
            "U" => Some(Self::U),
            "R" => Some(Self::R),
            "U'" => Some(Self::Up),
            "R'" => Some(Self::Rp),
            _ => None
        }
    }

    pub fn parse_array(str: &str) -> Option<Vec<Self>> {
        let mut cube_moves = vec![];
        let splits = str.split(' ');

        for split in splits {
            match Self::parse(split) {
                Some(cube_move) => cube_moves.push(cube_move),
                None => return None,
            }
        }

        Some(cube_moves)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cube {
    cubies: HashMap<CubePieceLocation, CubePiece>,
}

impl Cube {
    pub fn apply_moves(&mut self, moves: &Vec<CubeMove>) {
        for cube_move in moves {
            self.apply_move(*cube_move);
        }
    }

    fn apply_move(&mut self, cube_move: CubeMove) {
        match cube_move {
            CubeMove::U => {
                self.cycle_cubies(&CYCLE_U_CORNERS, &TWIST_CORNERS_SOLVED);
                self.cycle_cubies(&CYCLE_U_EDGES, &TWIST_EDGES_SOLVED);
            }
            CubeMove::R => {
                self.cycle_cubies(&CYCLE_R_CORNERS, &TWIST_CORNERS_120_240);
                self.cycle_cubies(&CYCLE_R_EDGES, &TWIST_EDGES_FLIP);
            }
            CubeMove::Up => {
                self.cycle_cubies(&CYCLE_UP_CORNERS, &TWIST_CORNERS_SOLVED);
                self.cycle_cubies(&CYCLE_UP_EDGES, &TWIST_EDGES_SOLVED);
            }
            CubeMove::Rp => {
                self.cycle_cubies(&CYCLE_RP_CORNERS, &TWIST_CORNERS_120_240);
                self.cycle_cubies(&CYCLE_RP_EDGES, &TWIST_EDGES_FLIP);
            }
        }
    }

    fn cycle_cubies(
        &mut self,
        locations: &'static [CubePieceLocation; 4],
        twists: &'static [Twist; 4],
    ) {
        let first_position = locations[0];
        let mut last_cubie = self.cubies[&first_position];

        for (target_position, twist) in locations
            .iter()
            .skip(1)
            .chain(once(&first_position))
            .zip(twists)
        {
            let target_cubie = self.cubies[target_position];

            // save which one was here before permutation
            let cubie_to_send = last_cubie;
            last_cubie = target_cubie;

            // do the permutation
            self.cubies
                .insert(*target_position, cubie_to_send.twisted(*twist));
        }
    }

    pub fn is_solved(&self) -> bool {
        for (location, expected_cubie) in &SOLVED_CUBIES {
            if self.cubies[location] != *expected_cubie {
                return false;
            }
        }

        true
    }

    pub fn iter_corners(&self) -> impl Iterator<Item = (&CubePieceLocation, &CubePiece)> {
        self.cubies.iter().filter(|(_, piece)| piece.is_corner())
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (&CubePieceLocation, &CubePiece)> {
        self.cubies.iter().filter(|(_, piece)| piece.is_edge())
    }

    pub fn get_piece_at(&self, sticker_location: &CubePieceLocation) -> CubePiece {
        self.cubies[sticker_location]
    }

    pub fn get_sticker_origin(
        &self,
        sticker_location: &CubeStickerLocation,
    ) -> CubeStickerLocation {
        let piece = self.cubies[&sticker_location.piece_location];
        CubeStickerLocation {
            piece_location: piece.get_original_location(),
            twist: piece.get_opposite_twist(),
        }
    }

    pub fn solved() -> Cube {
        Cube {
            cubies: HashMap::from(SOLVED_CUBIES),
        }
    }

    pub fn random_uniform<R: Rng>(rng: &mut R) -> Cube {
        // Generate permutations of corners and edges
        let corners_permutation = permutations::random_uniform_permutation::<8, R>(rng);
        let mut edges_permutation = permutations::random_uniform_permutation::<12, R>(rng);

        // Check permutatation parity
        if permutations::permutation_parity(&corners_permutation) != permutations::permutation_parity(&edges_permutation) {
            edges_permutation.swap(0, 1);
        }

        let solved_cubies = HashMap::from(SOLVED_CUBIES);
        let mut cubies: HashMap<CubePieceLocation, CubePiece> = HashMap::with_capacity( 8 + 12);

        // Generate random twists
        let mut corner_twists: [Twist; 8] = std::array::from_fn(|_| Twist::random_uniform_corner(rng));
        let mut edge_twists: [Twist; 12] = std::array::from_fn(|_| Twist::random_uniform_edge(rng));

        let corner_twist_sum = corner_twists
            .iter()
            .copied()
            .reduce(|sum, twist| twist.corner_add(sum))
            .unwrap();
        let edge_twist_sum = edge_twists
            .iter()
            .copied()
            .reduce(|sum, twist| twist.edge_add(sum))
            .unwrap();

        // Correct twists
        corner_twists[0] = corner_twists[0].corner_add(corner_twist_sum.corner_opposite());
        edge_twists[0] = edge_twists[0].edge_add(edge_twist_sum.edge_opposite());

        // Make cube
        for corner_index in corners_permutation {
            let location = CORNER_LOCATIONS[corner_index];
            let twist = corner_twists[corner_index];
            cubies.insert(location, solved_cubies[&location].twisted(twist));
        }

        for edge_index in edges_permutation {
            let location = EDGE_LOCATIONS[edge_index];
            let twist = edge_twists[edge_index];
            cubies.insert(location, solved_cubies[&location].twisted(twist));
        }

        Cube { cubies }
    }
}

static CORNER_LOCATIONS: [CubePieceLocation; 8] = [
    CubePieceLocation::UFR,
    CubePieceLocation::UFL,
    CubePieceLocation::UBL,
    CubePieceLocation::UBR,
    CubePieceLocation::DFR,
    CubePieceLocation::DFL,
    CubePieceLocation::DBL,
    CubePieceLocation::DBR,
];

static EDGE_LOCATIONS: [CubePieceLocation; 12] = [
    CubePieceLocation::UR,
    CubePieceLocation::UF,
    CubePieceLocation::UL,
    CubePieceLocation::UB,
    CubePieceLocation::DR,
    CubePieceLocation::DF,
    CubePieceLocation::DL,
    CubePieceLocation::DB,
    CubePieceLocation::FR,
    CubePieceLocation::FL,
    CubePieceLocation::BL,
    CubePieceLocation::BR,
];


static SOLVED_CUBIES: [(CubePieceLocation, CubePiece); 20] = [
    (CubePieceLocation::UFR, CubePiece::UFR),
    (CubePieceLocation::UFL, CubePiece::UFL),
    (CubePieceLocation::UBL, CubePiece::UBL),
    (CubePieceLocation::UBR, CubePiece::UBR),
    (CubePieceLocation::DFR, CubePiece::DFR),
    (CubePieceLocation::DFL, CubePiece::DFL),
    (CubePieceLocation::DBL, CubePiece::DBL),
    (CubePieceLocation::DBR, CubePiece::DBR),
    (CubePieceLocation::UR, CubePiece::UR),
    (CubePieceLocation::UF, CubePiece::UF),
    (CubePieceLocation::UL, CubePiece::UL),
    (CubePieceLocation::UB, CubePiece::UB),
    (CubePieceLocation::DR, CubePiece::DR),
    (CubePieceLocation::DF, CubePiece::DF),
    (CubePieceLocation::DL, CubePiece::DL),
    (CubePieceLocation::DB, CubePiece::DB),
    (CubePieceLocation::FR, CubePiece::FR),
    (CubePieceLocation::FL, CubePiece::FL),
    (CubePieceLocation::BL, CubePiece::BL),
    (CubePieceLocation::BR, CubePiece::BR),
];

// ? We could newtype an create CornerCycle and EdgeCycle and validate at compile time that there is no faces/edges/corners in same array
static CYCLE_U_CORNERS: [CubePieceLocation; 4] = [
    CubePieceLocation::UFL,
    CubePieceLocation::UBL,
    CubePieceLocation::UBR,
    CubePieceLocation::UFR,
];
static CYCLE_U_EDGES: [CubePieceLocation; 4] = [
    CubePieceLocation::UF,
    CubePieceLocation::UL,
    CubePieceLocation::UB,
    CubePieceLocation::UR,
];
static CYCLE_R_CORNERS: [CubePieceLocation; 4] = [
    CubePieceLocation::UFR,
    CubePieceLocation::UBR,
    CubePieceLocation::DBR,
    CubePieceLocation::DFR,
];
static CYCLE_R_EDGES: [CubePieceLocation; 4] = [
    CubePieceLocation::UR,
    CubePieceLocation::BR,
    CubePieceLocation::DR,
    CubePieceLocation::FR,
];
static CYCLE_UP_CORNERS: [CubePieceLocation; 4] = [
    CubePieceLocation::UFL,
    CubePieceLocation::UFR,
    CubePieceLocation::UBR,
    CubePieceLocation::UBL,
];
static CYCLE_UP_EDGES: [CubePieceLocation; 4] = [
    CubePieceLocation::UF,
    CubePieceLocation::UR,
    CubePieceLocation::UB,
    CubePieceLocation::UL,
];
static CYCLE_RP_CORNERS: [CubePieceLocation; 4] = [
    CubePieceLocation::UFR,
    CubePieceLocation::DFR,
    CubePieceLocation::DBR,
    CubePieceLocation::UBR,
];
static CYCLE_RP_EDGES: [CubePieceLocation; 4] = [
    CubePieceLocation::UR,
    CubePieceLocation::FR,
    CubePieceLocation::DR,
    CubePieceLocation::BR,
];
static TWIST_CORNERS_SOLVED: [Twist; 4] =
    [Twist::SOLVED, Twist::SOLVED, Twist::SOLVED, Twist::SOLVED];
static TWIST_CORNERS_120_240: [Twist; 4] =
    [Twist::CW_120, Twist::CW_240, Twist::CW_120, Twist::CW_240];
// static TWIST_CORNERS_240_120: [Twist; 4] = [Twist::CW_240, Twist::CW_120, Twist::CW_240, Twist::CW_120];
static TWIST_EDGES_SOLVED: [Twist; 4] =
    [Twist::SOLVED, Twist::SOLVED, Twist::SOLVED, Twist::SOLVED];
static TWIST_EDGES_FLIP: [Twist; 4] = [
    Twist::FLIPPED,
    Twist::FLIPPED,
    Twist::FLIPPED,
    Twist::FLIPPED,
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_u() {
        let mut cube = Cube::solved();
        cube.apply_move(CubeMove::U);

        assert_eq!(cube.cubies[&CubePieceLocation::UFL], CubePiece::UFR);
        assert_eq!(cube.cubies[&CubePieceLocation::UFR], CubePiece::UBR);
        assert_eq!(cube.cubies[&CubePieceLocation::UBR], CubePiece::UBL);
        assert_eq!(cube.cubies[&CubePieceLocation::UBL], CubePiece::UFL);

        assert_eq!(cube.cubies[&CubePieceLocation::UF], CubePiece::UR);
        assert_eq!(cube.cubies[&CubePieceLocation::UR], CubePiece::UB);
        assert_eq!(cube.cubies[&CubePieceLocation::UB], CubePiece::UL);
        assert_eq!(cube.cubies[&CubePieceLocation::UL], CubePiece::UF);
    }

    #[test]
    fn test_move_up() {
        let mut cube = Cube::solved();
        cube.apply_move(CubeMove::Up);

        assert_eq!(cube.cubies[&CubePieceLocation::UFR], CubePiece::UFL);
        assert_eq!(cube.cubies[&CubePieceLocation::UBR], CubePiece::UFR);
        assert_eq!(cube.cubies[&CubePieceLocation::UBL], CubePiece::UBR);
        assert_eq!(cube.cubies[&CubePieceLocation::UFL], CubePiece::UBL);

        assert_eq!(cube.cubies[&CubePieceLocation::UR], CubePiece::UF);
        assert_eq!(cube.cubies[&CubePieceLocation::UB], CubePiece::UR);
        assert_eq!(cube.cubies[&CubePieceLocation::UL], CubePiece::UB);
        assert_eq!(cube.cubies[&CubePieceLocation::UF], CubePiece::UL);
    }

    #[test]
    fn test_move_r() {
        let mut cube = Cube::solved();
        cube.apply_move(CubeMove::R);

        assert_eq!(
            cube.cubies[&CubePieceLocation::UBR],
            CubePiece::UFR.twisted(Twist::CW_120)
        );
        assert_eq!(
            cube.cubies[&CubePieceLocation::DBR],
            CubePiece::UBR.twisted(Twist::CW_240)
        );
        assert_eq!(
            cube.cubies[&CubePieceLocation::DFR],
            CubePiece::DBR.twisted(Twist::CW_120)
        );
        assert_eq!(
            cube.cubies[&CubePieceLocation::UFR],
            CubePiece::DFR.twisted(Twist::CW_240)
        );

        assert_eq!(
            cube.cubies[&CubePieceLocation::BR],
            CubePiece::UR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.cubies[&CubePieceLocation::DR],
            CubePiece::BR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.cubies[&CubePieceLocation::FR],
            CubePiece::DR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.cubies[&CubePieceLocation::UR],
            CubePiece::FR.twisted(Twist::FLIPPED)
        );
    }

    #[test]
    fn test_move_rp() {
        let mut cube = Cube::solved();
        cube.apply_move(CubeMove::Rp);

        assert_eq!(
            cube.cubies[&CubePieceLocation::UFR],
            CubePiece::UBR.twisted(Twist::CW_240)
        );
        assert_eq!(
            cube.cubies[&CubePieceLocation::UBR],
            CubePiece::DBR.twisted(Twist::CW_120)
        );
        assert_eq!(
            cube.cubies[&CubePieceLocation::DBR],
            CubePiece::DFR.twisted(Twist::CW_240)
        );
        assert_eq!(
            cube.cubies[&CubePieceLocation::DFR],
            CubePiece::UFR.twisted(Twist::CW_120)
        );

        assert_eq!(
            cube.cubies[&CubePieceLocation::UR],
            CubePiece::BR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.cubies[&CubePieceLocation::BR],
            CubePiece::DR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.cubies[&CubePieceLocation::DR],
            CubePiece::FR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.cubies[&CubePieceLocation::FR],
            CubePiece::UR.twisted(Twist::FLIPPED)
        );
    }

    #[test]
    fn test_twist_r2() {
        use CubeMove::*;

        let mut cube = Cube::solved();
        cube.apply_move(R);
        cube.apply_move(R);

        assert!(
            cube.iter_corners()
                .all(|(_, c)| c.get_twist() == Twist::SOLVED)
        );
    }

    #[test]
    fn test_r4() {
        use CubeMove::*;

        let mut cube = Cube::solved();
        cube.apply_move(R);
        cube.apply_move(R);
        cube.apply_move(R);
        cube.apply_move(R);

        assert!(cube.is_solved());
    }

    #[test]
    fn test_r_rp() {
        use CubeMove::*;

        let mut cube = Cube::solved();
        cube.apply_move(R);
        cube.apply_move(Rp);

        assert!(cube.is_solved());
    }

    #[test]
    fn test_3_sexy_twist() {
        use CubeMove::*;

        let mut cube = Cube::solved();
        cube.apply_moves(&vec![R, U, Rp, Up, R, U, Rp, Up, R, U, Rp, Up]);

        assert_eq!(
            cube.get_piece_at(&CubePieceLocation::UFR).get_twist(),
            Twist::SOLVED
        );
    }

    #[test]
    fn test_3_sexy_has_no_effect() {
        use CubeMove::*;

        let mut cube = Cube::solved();
        cube.apply_moves(&vec![
            R, U, Rp, Up, R, U, Rp, Up, R, U, Rp, Up, R, U, Rp, Up, R, U, Rp, Up, R, U, Rp, Up,
        ]);
        assert_eq!(cube, Cube::solved());
    }

    #[test]
    fn test_random_cube_twist_sum() {
        let mut rng = rand::rng();

        for _ in 0..1000 {
            let cube = Cube::random_uniform(&mut rng);
    
            let corner_twist = cube.iter_corners().fold(Twist::SOLVED, |sum, (_, corner)| sum.corner_add(corner.get_twist()));
            assert_eq!(corner_twist, Twist::SOLVED);
        
            let edge_twist = cube.iter_edges().fold(Twist::SOLVED, |sum, (_, edge)| sum.edge_add(edge.get_twist()));
            assert_eq!(edge_twist, Twist::SOLVED);
        }
    }
}
