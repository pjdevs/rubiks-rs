use crate::location::PieceLocation;
use crate::moves::CubeMove;
use crate::piece::{Corner, CornerState, CornerTwist, Edge, EdgeState, EdgeTwist, Twist};
use crate::stickers::{CornerSticker, EdgeSticker};
use std::iter::once;

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

    pub fn iter_corners(&self) -> impl Iterator<Item = (PieceLocation, CornerState)> {
        self.corner_locations
            .iter()
            .enumerate()
            .map(|(index, location)| {
                let actual_piece_index = location.into_index();
                (
                    PieceLocation::from_index(index),
                    CornerState {
                        piece: Self::CORNER_PIECES[actual_piece_index],
                        twist: self.corner_states[actual_piece_index],
                    },
                )
            })
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = (PieceLocation, EdgeState)> {
        self.edge_locations
            .iter()
            .enumerate()
            .map(|(index, location)| {
                let actual_piece_index = location.into_index();
                (
                    PieceLocation::from_index(index),
                    EdgeState {
                        piece: Self::EDGE_PIECES[actual_piece_index],
                        twist: self.edge_states[actual_piece_index],
                    },
                )
            })
    }

    pub fn get_corner_sticker_name(&self, sticker: CornerSticker) -> String {
        let corner = self.get_corner_state_at(sticker.location);
        let combined_twist = corner.twist.add(&sticker.twist);

        let mut faces = corner.piece.get_faces();
        faces.rotate_right(combined_twist.number_of_twists());

        faces.iter().map(|face| face.to_string()).collect()
    }

    pub fn get_edge_sticker_name(&self, sticker: EdgeSticker) -> String {
        let edge = self.get_edge_state_at(sticker.location);
        let combined_twist = edge.twist.add(&sticker.twist);

        let mut faces = edge.piece.get_faces();
        faces.rotate_right(combined_twist.number_of_twists());

        faces.iter().map(|face| face.to_string()).collect()
    }

    pub fn get_corner_state_at(&self, location: PieceLocation) -> CornerState {
        let location_index = location.into_index();
        let original_piece_index = self.corner_locations[location_index].into_index();

        CornerState {
            piece: Self::CORNER_PIECES[original_piece_index],
            twist: self.corner_states[original_piece_index],
        }
    }

    pub fn get_edge_state_at(&self, location: PieceLocation) -> EdgeState {
        let location_index = location.into_index();
        let original_piece_index = self.corner_locations[location_index].into_index();

        EdgeState {
            piece: Self::EDGE_PIECES[original_piece_index],
            twist: self.edge_states[original_piece_index],
        }
    }

    pub fn apply_moves(&mut self, moves: &Vec<CubeMove>) {
        for cube_move in moves {
            self.apply_move(*cube_move);
        }
    }

    pub fn apply_move(&mut self, cube_move: CubeMove) {
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

    /// Set the corner which origin is at `piece` at `target_location` with twist `twist_to_apply` added.
    const fn set_corner_at(
        &mut self,
        target_location: PieceLocation,
        piece: PieceLocation,
        twist: CornerTwist,
    ) {
        self.corner_locations[target_location.into_index()] = piece;
        self.corner_states[piece.into_index()] = twist;
    }

    const fn set_edge_at(
        &mut self,
        target_location: PieceLocation,
        piece: PieceLocation,
        twist: EdgeTwist,
    ) {
        self.edge_locations[target_location.into_index()] = piece;
        self.edge_states[piece.into_index()] = twist;
    }

    fn cycle_corners(
        &mut self,
        locations: &'static [PieceLocation],
        twists: &'static [CornerTwist],
    ) {
        let first_position = locations[0];
        let mut last_piece = self.corner_locations[first_position.into_index()];

        for (target_position, twist) in locations
            .iter()
            .skip(1)
            .chain(once(&first_position))
            .zip(twists)
        {
            // save which one was here before permutation
            let piece_to_send = last_piece;
            last_piece = *target_position;

            // do the permutation
            self.set_corner_at(*target_position, piece_to_send, *twist);
        }
    }

    fn cycle_edges(
        &mut self,
        locations: &'static [PieceLocation],
        twists: &'static [EdgeTwist],
    ) {
        let first_position = locations[0];
        let mut last_piece = self.corner_locations[first_position.into_index()];

        for (target_position, twist) in locations
            .iter()
            .skip(1)
            .chain(once(&first_position))
            .zip(twists)
        {
            let target_piece = self.get_edge_state_at(*target_position);

            // save which one was here before permutation
            let cubie_to_send = last_piece;
            last_piece = target_piece;

            // do the permutation
            self.set_edge_at(*target_position, cubie_to_send, twist);
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
static TWIST_CORNERS_SOLVED: [CornerTwist; 4] = [
    Twist::solved(),
    Twist::solved(),
    Twist::solved(),
    Twist::solved(),
];
static TWIST_CORNERS_120_240: [CornerTwist; 4] =
    [Twist::CW_120, Twist::CW_240, Twist::CW_120, Twist::CW_240];
// static TWIST_CORNERS_240_120: [Twist; 4] = [Twist::CW_240, Twist::CW_120, Twist::CW_240, Twist::CW_120];
static TWIST_EDGES_SOLVED: [EdgeTwist; 4] = [
    Twist::solved(),
    Twist::solved(),
    Twist::solved(),
    Twist::solved(),
];
static TWIST_EDGES_FLIP: [EdgeTwist; 4] = [
    Twist::solved().flipped(),
    Twist::solved().flipped(),
    Twist::solved().flipped(),
    Twist::solved().flipped(),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_u() {
        let mut cube = CubeState::solved();
        cube.apply_move(CubeMove::U);

        assert_eq!(
            cube.get_corner_state_at(PieceLocation::UFL),
            CornerState {
                piece: Corner::UFR,
                twist: Twist::solved()
            }
        );
        assert_eq!(
            cube.get_corner_state_at(PieceLocation::UFR),
            CornerState {
                piece: Corner::UBR,
                twist: Twist::solved()
            }
        );
        assert_eq!(
            cube.get_corner_state_at(PieceLocation::UBR),
            CornerState {
                piece: Corner::UBL,
                twist: Twist::solved()
            }
        );
        assert_eq!(
            cube.get_corner_state_at(PieceLocation::UBL),
            CornerState {
                piece: Corner::UFL,
                twist: Twist::solved()
            }
        );

        assert_eq!(
            cube.get_edge_state_at(PieceLocation::UF),
            EdgeState {
                piece: Edge::UR,
                twist: Twist::solved()
            }
        );
        assert_eq!(
            cube.get_edge_state_at(PieceLocation::UR),
            EdgeState {
                piece: Edge::UB,
                twist: Twist::solved()
            }
        );
        assert_eq!(
            cube.get_edge_state_at(PieceLocation::UB),
            EdgeState {
                piece: Edge::UL,
                twist: Twist::solved()
            }
        );
        assert_eq!(
            cube.get_edge_state_at(PieceLocation::UL),
            EdgeState {
                piece: Edge::UF,
                twist: Twist::solved()
            }
        );
    }

    #[test]
    fn test_move_up() {
        let mut cube = CubeState::solved();
        cube.apply_move(CubeMove::Up);

        assert_eq!(
            cube.get_corner_state_at(PieceLocation::UFR),
            CornerState {
                piece: Corner::UFL,
                twist: Twist::solved()
            }
        );
        assert_eq!(
            cube.get_corner_state_at(PieceLocation::UBR),
            CornerState {
                piece: Corner::UFR,
                twist: Twist::solved()
            }
        );
        assert_eq!(
            cube.get_corner_state_at(PieceLocation::UBL),
            CornerState {
                piece: Corner::UBR,
                twist: Twist::solved()
            }
        );
        assert_eq!(
            cube.get_corner_state_at(PieceLocation::UFL),
            CornerState {
                piece: Corner::UBL,
                twist: Twist::solved()
            }
        );

        assert_eq!(
            cube.get_edge_state_at(PieceLocation::UR),
            EdgeState {
                piece: Edge::UF,
                twist: Twist::solved()
            }
        );
        assert_eq!(
            cube.get_edge_state_at(PieceLocation::UB),
            EdgeState {
                piece: Edge::UR,
                twist: Twist::solved()
            }
        );
        assert_eq!(
            cube.get_edge_state_at(PieceLocation::UL),
            EdgeState {
                piece: Edge::UB,
                twist: Twist::solved()
            }
        );
        assert_eq!(
            cube.get_edge_state_at(PieceLocation::UF),
            EdgeState {
                piece: Edge::UL,
                twist: Twist::solved()
            }
        );
    }

    #[test]
    fn test_move_r() {
        let mut cube = CubeState::solved();
        cube.apply_move(CubeMove::R);

        assert_eq!(
            cube.get_corner_state_at(PieceLocation::UBR),
            CornerState {
                piece: Corner::UFR,
                twist: CornerTwist::CW_120
            }
        );
        assert_eq!(
            cube.get_corner_state_at(PieceLocation::DBR),
            CornerState {
                piece: Corner::UBR,
                twist: CornerTwist::CW_240
            }
        );
        assert_eq!(
            cube.get_corner_state_at(PieceLocation::DFR),
            CornerState {
                piece: Corner::DBR,
                twist: CornerTwist::CW_120
            }
        );
        assert_eq!(
            cube.get_corner_state_at(PieceLocation::UFR),
            CornerState {
                piece: Corner::DFR,
                twist: CornerTwist::CW_240
            }
        );

        assert_eq!(
            cube.get_edge_state_at(PieceLocation::BR),
            EdgeState {
                piece: Edge::UR,
                twist: EdgeTwist::solved().flipped()
            }
        );
        assert_eq!(
            cube.get_edge_state_at(PieceLocation::DR),
            EdgeState {
                piece: Edge::BR,
                twist: EdgeTwist::solved().flipped()
            }
        );
        assert_eq!(
            cube.get_edge_state_at(PieceLocation::FR),
            EdgeState {
                piece: Edge::DR,
                twist: EdgeTwist::solved().flipped()
            }
        );
        assert_eq!(
            cube.get_edge_state_at(PieceLocation::UR),
            EdgeState {
                piece: Edge::FR,
                twist: EdgeTwist::solved().flipped()
            }
        );
    }

    #[test]
    fn test_move_rp() {
        let mut cube = CubeState::solved();
        cube.apply_move(CubeMove::Rp);

        assert_eq!(
            cube.get_corner_state_at(PieceLocation::UFR),
            CornerState {
                piece: Corner::UBR,
                twist: CornerTwist::CW_240
            }
        );
        assert_eq!(
            cube.get_corner_state_at(PieceLocation::UBR),
            CornerState {
                piece: Corner::DBR,
                twist: CornerTwist::CW_120
            }
        );
        assert_eq!(
            cube.get_corner_state_at(PieceLocation::DBR),
            CornerState {
                piece: Corner::DFR,
                twist: CornerTwist::CW_240
            }
        );
        assert_eq!(
            cube.get_corner_state_at(PieceLocation::DFR),
            CornerState {
                piece: Corner::UFR,
                twist: CornerTwist::CW_120
            }
        );

        assert_eq!(
            cube.get_edge_state_at(PieceLocation::UR),
            EdgeState {
                piece: Edge::BR,
                twist: Twist::solved().flipped()
            }
        );
        assert_eq!(
            cube.get_edge_state_at(PieceLocation::BR),
            EdgeState {
                piece: Edge::DR,
                twist: Twist::solved().flipped()
            }
        );
        assert_eq!(
            cube.get_edge_state_at(PieceLocation::DR),
            EdgeState {
                piece: Edge::FR,
                twist: Twist::solved().flipped()
            }
        );
        assert_eq!(
            cube.get_edge_state_at(PieceLocation::FR),
            EdgeState {
                piece: Edge::UR,
                twist: Twist::solved().flipped()
            }
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
                .all(|(_, state)| state.twist == Twist::solved())
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
            cube.get_corner_state_at(PieceLocation::UFR).twist,
            Twist::solved()
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
