use crate::cube_constants::{CORNER_NUMBER, EDGE_NUMBER};
use crate::location::{self, PieceLocation};
use crate::piece::{Corner, Edge, Piece, PieceState};
use crate::twist::Twist;
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubeState {
    corner_locations: [PieceLocation; Self::CORNER_NUMBER],
    corner_twists: [Twist; Self::CORNER_NUMBER],
    edge_locations: [PieceLocation; Self::EDGE_NUMBER],
    edge_twists: [Twist; Self::CORNER_NUMBER],
}

impl CubeState {
    pub const STICKERS_ON_CORNERS: u8 = 3;
    pub const STICKERS_ON_EDGES: u8 = 2;
    pub const EDGE_NUMBER: usize = 12;
    pub const CORNER_NUMBER: usize = 8;

    pub const CORNER_PIECES: [Corner; 8] = [
        Corner::UFR,
        Corner::UFL,
        Corner::UBR,
        Corner::UBL,
        Corner::DFR,
        Corner::DFL,
        Corner::DBR,
        Corner::DLB,
    ];

    pub const EDGE_PIECES: [Edge; 12] = [
        Edge::UF,
        Edge::UL,
        Edge::UB,
        Edge::UR,
        Edge::DF,
        Edge::DL,
        Edge::DB,
        Edge::DR,
        Edge::FR,
        Edge::FL,
        Edge::BR,
        Edge::BL,
    ];

    pub fn apply_moves(&mut self, moves: &Vec<CubeMove>) {
        for cube_move in moves {
            self.apply_move(*cube_move);
        }
    }

    fn apply_move(&mut self, cube_move: CubeMove) {
        match cube_move {
            CubeMove::U => {
                self.cycle_pieces(&CYCLE_U_CORNERS, &TWIST_CORNERS_SOLVED);
                self.cycle_pieces(&CYCLE_U_EDGES, &TWIST_EDGES_SOLVED);
            }
            CubeMove::R => {
                self.cycle_pieces(&CYCLE_R_CORNERS, &TWIST_CORNERS_120_240);
                self.cycle_pieces(&CYCLE_R_EDGES, &TWIST_EDGES_FLIP);
            }
            CubeMove::Up => {
                self.cycle_pieces(&CYCLE_UP_CORNERS, &TWIST_CORNERS_SOLVED);
                self.cycle_pieces(&CYCLE_UP_EDGES, &TWIST_EDGES_SOLVED);
            }
            CubeMove::Rp => {
                self.cycle_pieces(&CYCLE_RP_CORNERS, &TWIST_CORNERS_120_240);
                self.cycle_pieces(&CYCLE_RP_EDGES, &TWIST_EDGES_FLIP);
            }
        }
    }

    fn cycle_pieces(
        &mut self,
        locations: &'static [PieceLocation; 4],
        twists: &'static [Twist; 4],
    ) {
        let first_position = locations[0];
        let mut last_piece = self.get_piece_at(&first_position);

        for (target_position, twist) in locations
            .iter()
            .skip(1)
            .chain(once(&first_position))
            .zip(twists)
        {
            let target_piece = self.get_piece_at(target_position);

            // save which one was here before permutation
            let cubie_to_send = last_piece;
            last_piece = target_piece;

            // do the permutation
            self.set_piece_at(target_position, cubie_to_send.twisted(*twist));
        }
    }

    pub const  fn is_solved(&self) -> bool {
        false
    }

    pub fn iter_corners(&self) -> impl Iterator<Item = (PieceLocation, CubePiece)> {
        self.corner_locations.iter().enumerate().map(|(index, piece)| { (PieceLocation::BL, *piece) })
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (PieceLocation, CubePiece)> {
        self.edge_locations.iter().enumerate().map(|(index, piece)| { (PieceLocation::BL, *piece) })
    }

    pub fn solved() -> CubeState {
        CubeState {
            corner_locations: [CubePiece::UFR; 8],
            edge_locations: [CubePiece::BL; 12],
        }
    }

    pub fn get_corner_at(&self, location_index: usize) -> PieceState<> {
        let piece_index = self.corner_positions[location_index];
        let orientation = self.corner_twists[location_index];
        (Self::CORNER_PIECES[piece_index], orientation)
    }

    pub fn get_edge_at(&self, location_index: usize) -> (Piece<2>, Twist) {
        let piece_index = self.edge_positions[location_index];
        let orientation = self.edge_twists[location_index];
        (Self::EDGE_PIECES[piece_index], orientation)
    }

    const fn set_piece_at(&mut self, sticker_location: &PieceLocation, piece: CubePiece) {
        let target_index = sticker_location.get_index();
        let old_piece = if piece.is_corner() {
            &mut self.corner_locations[target_index]
        } else {
            &mut self.edge_locations[target_index]
        };
        assert!(old_piece.is_corner() == piece.is_corner());
        *old_piece = piece;
    }

    pub fn get_sticker_origin(
        &self,
        sticker_location: &CubeStickerLocation,
    ) -> CubeStickerLocation {
        let piece = self.get_piece_at(&sticker_location.piece_location);
        CubeStickerLocation {
            piece_location: piece.get_original_location(),
            twist: piece.get_opposite_twist(),
        }
    }
}

// ? We could newtype an create CornerCycle and EdgeCycle and validate at compile time that there is no faces/edges/corners in same array
static CYCLE_U_CORNERS: [PieceLocation; 4] = [
    location::UFL,
    location::UBL,
    location::UBR,
    location::UFR,
];
static CYCLE_U_EDGES: [PieceLocation; 4] = [
    location::UF,
    location::UL,
    location::UB,
    location::UR,
];
static CYCLE_R_CORNERS: [PieceLocation; 4] = [
    location::UFR,
    location::UBR,
    location::DBR,
    location::DFR,
];
static CYCLE_R_EDGES: [PieceLocation; 4] = [
    location::UR,
    location::BR,
    location::DR,
    location::FR,
];
static CYCLE_UP_CORNERS: [PieceLocation; 4] = [
    location::UFL,
    location::UFR,
    location::UBR,
    location::UBL,
];
static CYCLE_UP_EDGES: [PieceLocation; 4] = [
    location::UF,
    location::UR,
    location::UB,
    location::UL,
];
static CYCLE_RP_CORNERS: [PieceLocation; 4] = [
    location::UFR,
    location::DFR,
    location::DBR,
    location::UBR,
];
static CYCLE_RP_EDGES: [PieceLocation; 4] = [
    location::UR,
    location::FR,
    location::DR,
    location::BR,
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
        let mut cube = CubeState::solved();
        cube.apply_move(CubeMove::U);

        assert_eq!(cube.get_piece_at(&location::UFL), CubePiece::UFR);
        assert_eq!(cube.get_piece_at(&location::UFR), CubePiece::UBR);
        assert_eq!(cube.get_piece_at(&location::UBR), CubePiece::UBL);
        assert_eq!(cube.get_piece_at(&location::UBL), CubePiece::UFL);

        assert_eq!(cube.get_piece_at(&location::UF), CubePiece::UR);
        assert_eq!(cube.get_piece_at(&location::UR), CubePiece::UB);
        assert_eq!(cube.get_piece_at(&location::UB), CubePiece::UL);
        assert_eq!(cube.get_piece_at(&location::UL), CubePiece::UF);
    }

    #[test]
    fn test_move_up() {
        let mut cube = CubeState::solved();
        cube.apply_move(CubeMove::Up);

        assert_eq!(cube.get_piece_at(&location::UFR), CubePiece::UFL);
        assert_eq!(cube.get_piece_at(&location::UBR), CubePiece::UFR);
        assert_eq!(cube.get_piece_at(&location::UBL), CubePiece::UBR);
        assert_eq!(cube.get_piece_at(&location::UFL), CubePiece::UBL);

        assert_eq!(cube.get_piece_at(&location::UR), CubePiece::UF);
        assert_eq!(cube.get_piece_at(&location::UB), CubePiece::UR);
        assert_eq!(cube.get_piece_at(&location::UL), CubePiece::UB);
        assert_eq!(cube.get_piece_at(&location::UF), CubePiece::UL);
    }

    #[test]
    fn test_move_r() {
        let mut cube = CubeState::solved();
        cube.apply_move(CubeMove::R);

        assert_eq!(
            cube.get_piece_at(&location::UBR),
            CubePiece::UFR.twisted(Twist::CW_120)
        );
        assert_eq!(
            cube..get_piece_at(&location::DBR),
            CubePiece::UBR.twisted(Twist::CW_240)
        );
        assert_eq!(
            cube.get_piece_at(&location::DFR),
            CubePiece::DBR.twisted(Twist::CW_120)
        );
        assert_eq!(
            cube.get_piece_at(&location::UFR),
            CubePiece::DFR.twisted(Twist::CW_240)
        );

        assert_eq!(
            cube.get_piece_at(&location::BR),
            CubePiece::UR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.get_piece_at(&location::DR),
            CubePiece::BR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.get_piece_at(&location::FR),
            CubePiece::DR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.get_piece_at(&location::UR),
            CubePiece::FR.twisted(Twist::FLIPPED)
        );
    }

    #[test]
    fn test_move_rp() {
        let mut cube = CubeState::solved();
        cube.apply_move(CubeMove::Rp);

        assert_eq!(
            cube.get_piece_at(&location::UFR),
            CubePiece::UBR.twisted(Twist::CW_240)
        );
        assert_eq!(
            cube.get_piece_at(&location::UBR),
            CubePiece::DBR.twisted(Twist::CW_120)
        );
        assert_eq!(
            cube.get_piece_at(&location::DBR),
            CubePiece::DFR.twisted(Twist::CW_240)
        );
        assert_eq!(
            cube.get_piece_at(&location::DFR),
            CubePiece::UFR.twisted(Twist::CW_120)
        );

        assert_eq!(
            cube.get_piece_at(&location::UR),
            CubePiece::BR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.get_piece_at(&location::BR),
            CubePiece::DR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.get_piece_at(&location::DR),
            CubePiece::FR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.get_piece_at(&location::FR),
            CubePiece::UR.twisted(Twist::FLIPPED)
        );
    }

    #[test]
    fn test_twist_r2() {
        use CubeMove::*;

        let mut cube = CubeState::solved();
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

        let mut cube = CubeState::solved();
        cube.apply_move(R);
        cube.apply_move(R);
        cube.apply_move(R);
        cube.apply_move(R);

        assert!(cube.is_solved());
    }

    #[test]
    fn test_r_rp() {
        use CubeMove::*;

        let mut cube = CubeState::solved();
        cube.apply_move(R);
        cube.apply_move(Rp);

        assert!(cube.is_solved());
    }

    #[test]
    fn test_3_sexy_twist() {
        use CubeMove::*;

        let mut cube = CubeState::solved();
        cube.apply_moves(&vec![R, U, Rp, Up, R, U, Rp, Up, R, U, Rp, Up]);

        assert_eq!(
            cube.get_piece_at(&location::UFR).get_twist(),
            Twist::SOLVED
        );
    }

    #[test]
    fn test_3_sexy_has_no_effect() {
        use CubeMove::*;

        let mut cube = CubeState::solved();
        cube.apply_moves(&vec![
            R, U, Rp, Up, R, U, Rp, Up, R, U, Rp, Up, R, U, Rp, Up, R, U, Rp, Up, R, U, Rp, Up,
        ]);
        assert_eq!(cube, CubeState::solved());
    }
}
