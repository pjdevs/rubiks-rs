use crate::location::PieceLocation;
use crate::moves::CubeMove;
use crate::piece::{Corner, CornerTwist, Edge, EdgeTwist, Twist};
use crate::stickers::{CornerSticker, EdgeSticker};
use std::iter::once;

enum CubePiece {
    Corner(Corner),
    Edge(),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubeState {
    corner_locations: [PieceLocation; Self::CORNER_NUMBER],
    corner_states: [CornerTwist; Self::CORNER_NUMBER],
    edge_locations: [PieceLocation; Self::EDGE_NUMBER],
    edge_states: [EdgeTwist; Self::EDGE_NUMBER],
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
                self.cycle_corners(&CYCLE_U_CORNERS, &TWIST_CORNERS_SOLVED);
                self.cycle_edges(&CYCLE_U_EDGES, &TWIST_EDGES_SOLVED);
            }
            CubeMove::R => {
                self.cycle_corners(&CYCLE_R_CORNERS, &TWIST_CORNERS_120_240);
                self.cycle_edges(&CYCLE_R_EDGES, &TWIST_EDGES_FLIP);
            }
            CubeMove::Up => {
                self.cycle_corners(&CYCLE_UP_CORNERS, &TWIST_CORNERS_SOLVED);
                self.cycle_edges(&CYCLE_UP_EDGES, &TWIST_EDGES_SOLVED);
            }
            CubeMove::Rp => {
                self.cycle_corners(&CYCLE_RP_CORNERS, &TWIST_CORNERS_120_240);
                self.cycle_edges(&CYCLE_RP_EDGES, &TWIST_EDGES_FLIP);
            }
        }
    }

    const fn cycle_corners(
        &mut self,
        locations: &'static [PieceLocation; 4],
        twists: &'static [CornerTwist; 4],
    ) {
        locations.rotate_right(1);

        let first_position = locations[0];
        let mut last_piece = self.get_corner_at(first_position);

        for (target_position, twist) in locations
            .iter()
            .skip(1)
            .chain(once(&first_position))
            .zip(twists)
        {
            let target_piece = self.get_corner_at(*target_position);

            // save which one was here before permutation
            let cubie_to_send = last_piece;
            last_piece = target_piece;

            // do the permutation
            self.set_corner_at(target_position, cubie_to_send.twisted(*twist));
        }
    }

    const fn cycle_edges(
        &mut self,
        locations: &'static [PieceLocation; 4],
        twists: &'static [CornerTwist; 4],
    ) {
        locations.rotate_right(1);

        let first_position = locations[0];
        let mut last_piece = self.get_edge_at(first_position);

        for (target_position, twist) in locations
            .iter()
            .skip(1)
            .chain(once(&first_position))
            .zip(twists)
        {
            let target_piece = self.get_edge_at(*target_position);

            // save which one was here before permutation
            let cubie_to_send = last_piece;
            last_piece = target_piece;

            // do the permutation
            self.set_piece_at(target_position, cubie_to_send.twisted(*twist));
        }
    }

    pub fn is_solved(&self) -> bool {
        self.corner_locations
            .iter()
            .enumerate()
            .all(|(index, location)| PieceLocation::from_index(index) == *location)
            && self
                .edge_locations
                .iter()
                .enumerate()
                .all(|(index, location)| PieceLocation::from_index(index) == *location)
            && self.corner_states.iter().all(|state| state.is_solved())
            && self.edge_states.iter().all(|state| state.is_solved())
    }

    pub fn iter_corners(&self) -> impl Iterator<Item = (PieceLocation, Corner)> {
        self.corner_locations
            .iter()
            .enumerate()
            .map(|(index, location)| {
                (
                    PieceLocation::from_index(index),
                    Self::CORNER_PIECES[location.into_index()],
                )
            })
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (PieceLocation, Edge)> {
        self.edge_locations
            .iter()
            .enumerate()
            .map(|(index, location)| {
                (
                    PieceLocation::from_index(index),
                    Self::EDGE_PIECES[location.into_index()],
                )
            })
    }

    pub const fn solved() -> CubeState {
        let mut corner_locations = [PieceLocation::from_index(0); Self::CORNER_NUMBER];
        let mut edge_locations = [PieceLocation::from_index(0); Self::EDGE_NUMBER];

        let i = 0;
        while i < Self::CORNER_NUMBER || i < Self::EDGE_NUMBER {
            if i < Self::CORNER_NUMBER {
                corner_locations[i] = PieceLocation::from_index(i);
            }
            if i < Self::EDGE_NUMBER {
                edge_locations[i] = PieceLocation::from_index(i);
            }
        }

        CubeState {
            corner_locations,
            corner_states: [CornerTwist::solved(); Self::CORNER_NUMBER],
            edge_locations,
            edge_states: [EdgeTwist::solved(); Self::EDGE_NUMBER],
        }
    }

    pub fn get_corner_at(&self, location: PieceLocation) -> (Corner, CornerTwist) {
        let location_index = location.into_index();
        let original_piece_index = self.corner_locations[location_index].into_index();
        (
            Self::CORNER_PIECES[original_piece_index],
            self.corner_states[original_piece_index],
        )
    }

    pub fn get_edge_at(&self, location: PieceLocation) -> (Edge, EdgeTwist) {
        let location_index = location.into_index();
        let original_piece_index = self.corner_locations[location_index].into_index();
        (
            Self::EDGE_PIECES[original_piece_index],
            self.edge_states[original_piece_index],
        )
    }

    /// Set the corner which origin is `piece_location` at `target_location` with twist `twist`.
    pub const fn set_corner_at(&mut self, target_location: PieceLocation, piece_location: PieceLocation, twist: CornerTwist) {
        self.corner_locations[target_location.into_index()] = piece_location;
        self.corner_states[piece_location.into_index()] = twist;
    }

    pub const fn set_edge_at(&mut self, target_location: PieceLocation, piece_location: PieceLocation, twist: EdgeTwist) {
        self.edge_locations[target_location.into_index()] = piece_location;
        self.edge_states[piece_location.into_index()] = twist;
    }

    pub fn get_corner_sticker_name(&self, sticker: CornerSticker) -> String {
        let (corner, corner_twist) = self.get_corner_at(sticker.location);
        let combined_twist = corner_twist.add(&sticker.twist);

        let mut faces = corner.get_faces();
        faces.rotate_right(combined_twist.number_of_twists());

        faces.iter().map(|face| { face.to_string() }).collect()
    }

    pub fn get_edge_sticker_name(&self, sticker: EdgeSticker) -> String {
        let (corner, corner_twist) = self.get_edge_at(sticker.location);
        let combined_twist = corner_twist.add(&sticker.twist);

        let mut faces = corner.get_faces();
        faces.rotate_right(combined_twist.number_of_twists());

        faces.iter().map(|face| { face.to_string() }).collect()
    }

    pub const fn get_sticker_origin(
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
    PieceLocation::UFL,
    PieceLocation::UBL,
    PieceLocation::UBR,
    PieceLocation::UFR,
];
static CYCLE_U_EDGES: [PieceLocation; 4] = [
    PieceLocation::UF,
    PieceLocation::UL,
    PieceLocation::UB,
    PieceLocation::UR,
];
static CYCLE_R_CORNERS: [PieceLocation; 4] = [
    PieceLocation::UFR,
    PieceLocation::UBR,
    PieceLocation::DBR,
    PieceLocation::DFR,
];
static CYCLE_R_EDGES: [PieceLocation; 4] = [
    PieceLocation::UR,
    PieceLocation::BR,
    PieceLocation::DR,
    PieceLocation::FR,
];
static CYCLE_UP_CORNERS: [PieceLocation; 4] = [
    PieceLocation::UFL,
    PieceLocation::UFR,
    PieceLocation::UBR,
    PieceLocation::UBL,
];
static CYCLE_UP_EDGES: [PieceLocation; 4] = [
    PieceLocation::UF,
    PieceLocation::UR,
    PieceLocation::UB,
    PieceLocation::UL,
];
static CYCLE_RP_CORNERS: [PieceLocation; 4] = [
    PieceLocation::UFR,
    PieceLocation::DFR,
    PieceLocation::DBR,
    PieceLocation::UBR,
];
static CYCLE_RP_EDGES: [PieceLocation; 4] = [
    PieceLocation::UR,
    PieceLocation::FR,
    PieceLocation::DR,
    PieceLocation::BR,
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

        assert_eq!(cube.get_piece_at(&PieceLocation::UFL), Piece::UFR);
        assert_eq!(cube.get_piece_at(&PieceLocation::UFR), Piece::UBR);
        assert_eq!(cube.get_piece_at(&PieceLocation::UBR), Piece::UBL);
        assert_eq!(cube.get_piece_at(&PieceLocation::UBL), Piece::UFL);

        assert_eq!(cube.get_piece_at(&PieceLocation::UF), Piece::UR);
        assert_eq!(cube.get_piece_at(&PieceLocation::UR), Piece::UB);
        assert_eq!(cube.get_piece_at(&PieceLocation::UB), Piece::UL);
        assert_eq!(cube.get_piece_at(&PieceLocation::UL), Piece::UF);
    }

    #[test]
    fn test_move_up() {
        let mut cube = CubeState::solved();
        cube.apply_move(CubeMove::Up);

        assert_eq!(cube.get_piece_at(&PieceLocation::UFR), Piece::UFL);
        assert_eq!(cube.get_piece_at(&PieceLocation::UBR), Piece::UFR);
        assert_eq!(cube.get_piece_at(&PieceLocation::UBL), Piece::UBR);
        assert_eq!(cube.get_piece_at(&PieceLocation::UFL), Piece::UBL);

        assert_eq!(cube.get_piece_at(&PieceLocation::UR), Piece::UF);
        assert_eq!(cube.get_piece_at(&PieceLocation::UB), Piece::UR);
        assert_eq!(cube.get_piece_at(&PieceLocation::UL), Piece::UB);
        assert_eq!(cube.get_piece_at(&PieceLocation::UF), Piece::UL);
    }

    #[test]
    fn test_move_r() {
        let mut cube = CubeState::solved();
        cube.apply_move(CubeMove::R);

        assert_eq!(
            cube.get_piece_at(&PieceLocation::UBR),
            Piece::UFR.twisted(Twist::CW_120)
        );
        assert_eq!(
            cube..get_piece_at(&PieceLocation::DBR),
            Piece::UBR.twisted(Twist::CW_240)
        );
        assert_eq!(
            cube.get_piece_at(&PieceLocation::DFR),
            Piece::DBR.twisted(Twist::CW_120)
        );
        assert_eq!(
            cube.get_piece_at(&PieceLocation::UFR),
            Piece::DFR.twisted(Twist::CW_240)
        );

        assert_eq!(
            cube.get_piece_at(&PieceLocation::BR),
            Piece::UR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.get_piece_at(&PieceLocation::DR),
            Piece::BR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.get_piece_at(&PieceLocation::FR),
            Piece::DR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.get_piece_at(&PieceLocation::UR),
            Piece::FR.twisted(Twist::FLIPPED)
        );
    }

    #[test]
    fn test_move_rp() {
        let mut cube = CubeState::solved();
        cube.apply_move(CubeMove::Rp);

        assert_eq!(
            cube.get_piece_at(&PieceLocation::UFR),
            Piece::UBR.twisted(Twist::CW_240)
        );
        assert_eq!(
            cube.get_piece_at(&PieceLocation::UBR),
            Piece::DBR.twisted(Twist::CW_120)
        );
        assert_eq!(
            cube.get_piece_at(&PieceLocation::DBR),
            Piece::DFR.twisted(Twist::CW_240)
        );
        assert_eq!(
            cube.get_piece_at(&PieceLocation::DFR),
            Piece::UFR.twisted(Twist::CW_120)
        );

        assert_eq!(
            cube.get_piece_at(&PieceLocation::UR),
            Piece::BR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.get_piece_at(&PieceLocation::BR),
            Piece::DR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.get_piece_at(&PieceLocation::DR),
            Piece::FR.twisted(Twist::FLIPPED)
        );
        assert_eq!(
            cube.get_piece_at(&PieceLocation::FR),
            Piece::UR.twisted(Twist::FLIPPED)
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
            cube.get_piece_at(&PieceLocation::UFR).get_twist(),
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
