use std::collections::HashMap;
use std::iter::once;

// use crate::rubiks::stickers::{CornerSticker, CubeSticker, EdgeSticker};
use crate::rubiks::cubie::Cubie;
use crate::rubiks::faces::FaceMask;
use crate::rubiks::twist::Twist;

// const CORNER_STICKERS: [[CornerSticker; STICKERS_ON_CORNERS as usize]; NUMBER_OF_CORNERS as usize] = [
//     [CornerSticker::UFR, CornerSticker::FRU, CornerSticker::RFU],
//     [CornerSticker::UFL, CornerSticker::FLU, CornerSticker::LUF],
//     [CornerSticker::UBR, CornerSticker::BRU, CornerSticker::RUB],
//     [CornerSticker::UBL, CornerSticker::BLU, CornerSticker::LUB],
//     [CornerSticker::DFR, CornerSticker::FRD, CornerSticker::RDF],
//     [CornerSticker::DFL, CornerSticker::FLD, CornerSticker::LDF],
//     [CornerSticker::DBR, CornerSticker::BRD, CornerSticker::RDB],
//     [CornerSticker::DBL, CornerSticker::BLD, CornerSticker::LDB],
// ];

// pub const EDGE_STICKERS: [[EdgeSticker; STICKERS_ON_EDGES as usize]; NUMBER_OF_EDGES as usize] = [
//     [EdgeSticker::UF, EdgeSticker::FU],
//     [EdgeSticker::UL, EdgeSticker::LU],
//     [EdgeSticker::UB, EdgeSticker::BU],
//     [EdgeSticker::UR, EdgeSticker::RU],
//     [EdgeSticker::DF, EdgeSticker::FD],
//     [EdgeSticker::DL, EdgeSticker::LD],
//     [EdgeSticker::DB, EdgeSticker::BD],
//     [EdgeSticker::DR, EdgeSticker::RD],
//     [EdgeSticker::FR, EdgeSticker::RF],
//     [EdgeSticker::FL, EdgeSticker::LF],
//     [EdgeSticker::BR, EdgeSticker::RB],
//     [EdgeSticker::BL, EdgeSticker::LB],
// ];



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeMove {
    U, R, Up, Rp
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
pub struct Cube {
    cubies: HashMap<FaceMask, Cubie>
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
            },
            CubeMove::R => {
                self.cycle_cubies(&CYCLE_R_CORNERS, &TWIST_CORNERS_120_240);
                self.cycle_cubies(&CYCLE_R_EDGES, &TWIST_EDGES_FLIP);
            },
            CubeMove::Up => {
                self.cycle_cubies(&CYCLE_UP_CORNERS, &TWIST_CORNERS_SOLVED);
                self.cycle_cubies(&CYCLE_UP_EDGES, &TWIST_EDGES_SOLVED);
            },
            CubeMove::Rp => {
                self.cycle_cubies(&CYCLE_RP_CORNERS, &TWIST_CORNERS_240_120);
                self.cycle_cubies(&CYCLE_RP_EDGES, &TWIST_EDGES_FLIP);
            },
        }
    }

    fn cycle_cubies(&mut self, locations: &'static [FaceMask; 4], twists: &'static [Twist; 4]) {
        // let first_cubie = self.cubies[&locations[0]].twisted(twists[3]);

        // self.cubies.insert(locations[0], self.cubies[&locations[3]].twisted(twists[2]));
        // self.cubies.insert(locations[3], self.cubies[&locations[2]].twisted(twists[1]));
        // self.cubies.insert(locations[2], self.cubies[&locations[1]].twisted(twists[0]));
        // self.cubies.insert(locations[1], first_cubie);

        // let source = locations[0];

        // for (destination, twist)

        let first_position = locations[0];
        let mut last_cubie = self.cubies[&first_position];
        
        for (target_position, twist) in locations.iter().skip(1).chain(once(&first_position)).zip(twists) {
            let target_cubie = self.cubies[target_position];
            
            // save which one was here before permutation
            let cubie_to_send = last_cubie;
            last_cubie = target_cubie;

            // do the permutation
            self.cubies.insert(*target_position, cubie_to_send.twisted(*twist));
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

    pub fn solved() -> Cube {
        Cube {
            cubies: HashMap::from(SOLVED_CUBIES)
        }
    }

    // pub fn get_sticker_at(&self, sticker_location: CubeSticker) -> CubeSticker {
    //     let piece_location = Cube::get_location_from(sticker_location)
    //         .expect("Sticker should have a valid location");
        
    //     let cubie = match piece_location {
    //         CubeLocation::CornerLocation(corner) => Cubie::CornerCubie(self.corners[corner]),  
    //         CubeLocation::EdgeLocation(edge) => Cubie::EdgeCubie(self.edges[edge]),
    //     };

    //     Cube::sticker_inside(&cubie)
    // }

    // // TODO take into account the request sticker!
    // fn sticker_inside(cubie: &Cubie) -> CubeSticker {
    //     match cubie {
    //         Cubie::CornerCubie(corner_cubie) => {
    //             let mask = corner_cubie.get_face_mask();
    //             let sticker = CornerSticker::try_from(mask).expect("Corner cubie mask should be a corner sticker");
    //             let oriented_sticker = sticker.twisted(corner_cubie.orientation);
    //             CubeSticker::CornerSticker(oriented_sticker)
    //         },
    //         Cubie::EdgeCubie(edge_cubie) => {
    //             let mask = edge_cubie.get_face_mask();
    //             let sticker = EdgeSticker::try_from(mask).expect("Edge cubie mask should be an edge sticker");
    //             let oriented_sticker = match edge_cubie.orientation {
    //                 EdgeOrientation::Solved => sticker,
    //                 EdgeOrientation::Flipped => sticker.inverted(),
    //             };
    //             CubeSticker::EdgeSticker(oriented_sticker)
    //         },
    //     }
    // }

    // fn get_location_from<TElement: CubeElement>(element: TElement) -> Option<CubeLocation> {
    //     match element.get_face_mask() {
    //         Faces::UFR => Some(CubeLocation::CornerLocation(Corner::UFR)),
    //         Faces::UFL => Some(CubeLocation::CornerLocation(Corner::UFL)),
    //         Faces::UBL => Some(CubeLocation::CornerLocation(Corner::UBL)),
    //         Faces::UBR => Some(CubeLocation::CornerLocation(Corner::UBR)),
    //         Faces::DFR => Some(CubeLocation::CornerLocation(Corner::DFR)),
    //         Faces::DFL => Some(CubeLocation::CornerLocation(Corner::DFL)),
    //         Faces::DBL => Some(CubeLocation::CornerLocation(Corner::UFL)),
    //         Faces::DBR => Some(CubeLocation::CornerLocation(Corner::DBR)),
    //         Faces::UB => Some(CubeLocation::EdgeLocation(Edge::UB)),
    //         Faces::UR => Some(CubeLocation::EdgeLocation(Edge::UR)),
    //         Faces::UF => Some(CubeLocation::EdgeLocation(Edge::UF)),
    //         Faces::UL => Some(CubeLocation::EdgeLocation(Edge::UL)),
    //         Faces::DB => Some(CubeLocation::EdgeLocation(Edge::DB)),
    //         Faces::DR => Some(CubeLocation::EdgeLocation(Edge::DR)),
    //         Faces::DF => Some(CubeLocation::EdgeLocation(Edge::DF)),
    //         Faces::DL => Some(CubeLocation::EdgeLocation(Edge::DL)),
    //         Faces::BR => Some(CubeLocation::EdgeLocation(Edge::BR)),
    //         Faces::BL => Some(CubeLocation::EdgeLocation(Edge::BL)),
    //         Faces::FR => Some(CubeLocation::EdgeLocation(Edge::FR)),
    //         Faces::FL => Some(CubeLocation::EdgeLocation(Edge::FL)),
    //         _ => None
    //     }
    // }
}

static SOLVED_CUBIES: [(FaceMask, Cubie); 20] = [
    (FaceMask::UFR, Cubie::UFR),
    (FaceMask::UFL, Cubie::UFL),
    (FaceMask::UBL, Cubie::UBL),
    (FaceMask::UBR, Cubie::UBR),
    (FaceMask::DFR, Cubie::DFR),
    (FaceMask::DFL, Cubie::DFL),
    (FaceMask::DBL, Cubie::DBL),
    (FaceMask::DBR, Cubie::DBR),
    (FaceMask::UR, Cubie::UR),
    (FaceMask::UF, Cubie::UF),
    (FaceMask::UL, Cubie::UL),
    (FaceMask::UB, Cubie::UB),
    (FaceMask::DR, Cubie::DR),
    (FaceMask::DF, Cubie::DF),
    (FaceMask::DL, Cubie::DL),
    (FaceMask::DB, Cubie::DB),
    (FaceMask::FR, Cubie::FR),
    (FaceMask::FL, Cubie::FL),
    (FaceMask::BL, Cubie::BL),
    (FaceMask::BR, Cubie::BR),
];

// ? We could newtype an create CornerCycle and EdgeCycle and validate at compile time that there is no faces/edges/corners in same array
static CYCLE_U_CORNERS: [FaceMask; 4] = [FaceMask::UFL, FaceMask::UBL, FaceMask::UBR, FaceMask::UFR];
static CYCLE_U_EDGES: [FaceMask; 4] = [FaceMask::UF, FaceMask::UL, FaceMask::UB, FaceMask::UR];
static CYCLE_R_CORNERS: [FaceMask; 4] = [FaceMask::UFR, FaceMask::UBR, FaceMask::DBR, FaceMask::DFR];
static CYCLE_R_EDGES: [FaceMask; 4] = [FaceMask::UR, FaceMask::BR, FaceMask::DR, FaceMask::FR];
static CYCLE_UP_CORNERS: [FaceMask; 4] = [FaceMask::UFL, FaceMask::UFR, FaceMask::UBR, FaceMask::UBL];
static CYCLE_UP_EDGES: [FaceMask; 4] = [FaceMask::UF, FaceMask::UR, FaceMask::UB, FaceMask::UL];
static CYCLE_RP_CORNERS: [FaceMask; 4] = [FaceMask::UFR, FaceMask::DFR, FaceMask::DBR, FaceMask::UBR];
static CYCLE_RP_EDGES: [FaceMask; 4] = [FaceMask::UR, FaceMask::FR, FaceMask::DR, FaceMask::BR];
static TWIST_CORNERS_SOLVED: [Twist; 4] = [Twist::SOLVED, Twist::SOLVED, Twist::SOLVED, Twist::SOLVED];
static TWIST_CORNERS_120_240: [Twist; 4] = [Twist::CW_120, Twist::CW_240, Twist::CW_120, Twist::CW_240];
static TWIST_CORNERS_240_120: [Twist; 4] = [Twist::CW_240, Twist::CW_120, Twist::CW_240, Twist::CW_120];
static TWIST_EDGES_SOLVED: [Twist; 4] = [Twist::SOLVED, Twist::SOLVED, Twist::SOLVED, Twist::SOLVED];
static TWIST_EDGES_FLIP: [Twist; 4] = [Twist::FLIPPED, Twist::FLIPPED, Twist::FLIPPED, Twist::FLIPPED];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_u() {
        let mut cube = Cube::solved();
        cube.apply_move(CubeMove::U);

        assert_eq!(cube.cubies[&FaceMask::UFL], Cubie::UFR);
        assert_eq!(cube.cubies[&FaceMask::UFR], Cubie::UBR);
        assert_eq!(cube.cubies[&FaceMask::UBR], Cubie::UBL);
        assert_eq!(cube.cubies[&FaceMask::UBL], Cubie::UFL);

        assert_eq!(cube.cubies[&FaceMask::UF], Cubie::UR);
        assert_eq!(cube.cubies[&FaceMask::UR], Cubie::UB);
        assert_eq!(cube.cubies[&FaceMask::UB], Cubie::UL);
        assert_eq!(cube.cubies[&FaceMask::UL], Cubie::UF);
    }

    #[test]
    fn test_move_up() {
        let mut cube = Cube::solved();
        cube.apply_move(CubeMove::Up);

        assert_eq!(cube.cubies[&FaceMask::UFR], Cubie::UFL);
        assert_eq!(cube.cubies[&FaceMask::UBR], Cubie::UFR);
        assert_eq!(cube.cubies[&FaceMask::UBL], Cubie::UBR);
        assert_eq!(cube.cubies[&FaceMask::UFL], Cubie::UBL);

        assert_eq!(cube.cubies[&FaceMask::UR], Cubie::UF);
        assert_eq!(cube.cubies[&FaceMask::UB], Cubie::UR);
        assert_eq!(cube.cubies[&FaceMask::UL], Cubie::UB);
        assert_eq!(cube.cubies[&FaceMask::UF], Cubie::UL);
    }

    #[test]
    fn test_move_r() {
        let mut cube = Cube::solved();
        cube.apply_move(CubeMove::R);

        assert_eq!(cube.cubies[&FaceMask::UBR], Cubie::UFR.twisted(Twist::CW_120));
        assert_eq!(cube.cubies[&FaceMask::DBR], Cubie::UBR.twisted(Twist::CW_240));
        assert_eq!(cube.cubies[&FaceMask::DFR], Cubie::DBR.twisted(Twist::CW_120));
        assert_eq!(cube.cubies[&FaceMask::UFR], Cubie::DFR.twisted(Twist::CW_240));

        assert_eq!(cube.cubies[&FaceMask::BR], Cubie::UR.twisted(Twist::FLIPPED));
        assert_eq!(cube.cubies[&FaceMask::DR], Cubie::BR.twisted(Twist::FLIPPED));
        assert_eq!(cube.cubies[&FaceMask::FR], Cubie::DR.twisted(Twist::FLIPPED));
        assert_eq!(cube.cubies[&FaceMask::UR], Cubie::FR.twisted(Twist::FLIPPED));
    }

        #[test]
    fn test_move_rp() {
        let mut cube = Cube::solved();
        cube.apply_move(CubeMove::Rp);

        assert_eq!(cube.cubies[&FaceMask::UFR], Cubie::UBR.twisted(Twist::CW_120));
        assert_eq!(cube.cubies[&FaceMask::UBR], Cubie::DBR.twisted(Twist::CW_240));
        assert_eq!(cube.cubies[&FaceMask::DBR], Cubie::DFR.twisted(Twist::CW_120));
        assert_eq!(cube.cubies[&FaceMask::DFR], Cubie::UFR.twisted(Twist::CW_240));

        assert_eq!(cube.cubies[&FaceMask::UR], Cubie::BR.twisted(Twist::FLIPPED));
        assert_eq!(cube.cubies[&FaceMask::BR], Cubie::DR.twisted(Twist::FLIPPED));
        assert_eq!(cube.cubies[&FaceMask::DR], Cubie::FR.twisted(Twist::FLIPPED));
        assert_eq!(cube.cubies[&FaceMask::FR], Cubie::UR.twisted(Twist::FLIPPED));
    }

    #[test]
    fn sexy_has_no_effect() {
        use CubeMove::*;

        let mut cube = Cube::solved();
        cube.apply_moves(&vec![
            R, U, Rp, Up,
            R, U, Rp, Up,
            R, U, Rp, Up,
            R, U, Rp, Up,
            R, U, Rp, Up,
            R, U, Rp, Up,
        ]);
        assert_eq!(cube, Cube::solved());
    }
}