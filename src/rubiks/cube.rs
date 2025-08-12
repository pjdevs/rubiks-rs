use std::collections::HashMap;
use std::iter::once;

// use crate::rubiks::stickers::{CornerSticker, CubeSticker, EdgeSticker};
use crate::rubiks::faces::{Faces, STICKERS_ON_CORNERS, STICKERS_ON_EDGES, NUMBER_OF_CORNERS, NUMBER_OF_EDGES};

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

pub type Twist = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cubie {
    faces: Faces,
    twist: Twist,
}

impl Cubie {
    pub const TWIST_SOLVED: Twist = 0;
    pub const TWIST_120: Twist = 1;
    pub const TWIST_240: Twist = 2;
    pub const TWIST_FLIPPED: Twist = 1;

    pub const fn is_corner(&self) -> bool {
        self.faces.is_corner()
    }

    pub const fn is_edge(&self) -> bool {
        self.faces.is_edge()
    }

    pub const fn twisted(&self, twist: Twist) -> Cubie {
        let max_stickers = if self.is_corner() {
            STICKERS_ON_CORNERS
        } else {
            STICKERS_ON_EDGES
        };

        Cubie {
            faces: self.faces,
            twist: (self.twist + twist) % max_stickers
        }
    }

    pub const fn corner(faces: Faces) -> Result<Cubie, ()> {
        if !faces.is_corner() {
            return Err(())
        }

        Ok(Cubie {
            faces,
            twist: Self::TWIST_SOLVED
        })
    }

    pub const fn edge(faces: Faces) -> Result<Cubie, ()> {
        if !faces.is_edge() {
            return Err(())
        }

        Ok(Cubie {
            faces,
            twist: Self::TWIST_SOLVED
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeMove {
    U, R, Up, Rp
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cube {
    cubies: HashMap<Faces, Cubie>
}

impl Cube {
    pub fn apply_moves(&mut self, moves: &Vec<CubeMove>) {
        for cube_move in moves {
            self.apply_move(cube_move);
        }
    }

    fn apply_move(&mut self, cube_move: &CubeMove) {
        match cube_move {
            CubeMove::U => {
                self.cycle_cubies(
                    &[Faces::UFL, Faces::UBL, Faces::UBR, Faces::UFR],
                    &[Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED]
                );
                self.cycle_cubies(
                    &[Faces::UF, Faces::UL, Faces::UB, Faces::UR],
                    &[Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED]
                );
            },
            CubeMove::R => {
                self.cycle_cubies(
                    &[Faces::UFR, Faces::UBR, Faces::DBR, Faces::DFR],
                    &[Cubie::TWIST_120, Cubie::TWIST_240, Cubie::TWIST_120, Cubie::TWIST_240]
                );
                self.cycle_cubies(
                    &[Faces::UR, Faces::BR, Faces::DR, Faces::FR],
                    &[Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED]
                );
            },
            CubeMove::Up => {
                self.cycle_cubies(
                    &[Faces::UFL, Faces::UFR, Faces::UBR, Faces::UBL],
                    &[Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED]
                );
                self.cycle_cubies(
                    &[Faces::UF, Faces::UR, Faces::UB, Faces::UL],
                    &[Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED]
                );
            },
            CubeMove::Rp => {
                self.cycle_cubies(
                    &[Faces::UFR, Faces::DFR, Faces::DBR, Faces::UBR],
                    &[Cubie::TWIST_240, Cubie::TWIST_120, Cubie::TWIST_240, Cubie::TWIST_120]
                );
                self.cycle_cubies(
                    &[Faces::UR, Faces::FR, Faces::DR, Faces::BR],
                    &[Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED, Cubie::TWIST_SOLVED]
                );
            },
        }
    }

    fn cycle_cubies(&mut self, corners: &[Faces], twists: &[u8]) {
        let first_position = corners.first().expect("Corner cycles should be at least of size 1");
        let mut last_cubie = *self.cubies.get(first_position).expect("All pieces should be in cubies");
        
        for (target_position, twist) in corners.iter().skip(1).chain(once(first_position)).zip(twists) {
            let target_cubie = self.cubies.get(target_position).expect("All pieces should be in cubies");
            
            // save which one was here before permutation
            let cubie_to_send = last_cubie;
            last_cubie = *target_cubie;

            // do the permutation
            self.cubies.insert(*target_position, cubie_to_send.twisted(*twist));
        }
    }

    pub fn solved() -> Cube {
        Cube {
            cubies: HashMap::from([
                (Faces::UFR, Cubie::corner(Faces::UFR).unwrap()),
                (Faces::UFL, Cubie::corner(Faces::UFL).unwrap()),
                (Faces::UBL, Cubie::corner(Faces::UBL).unwrap()),
                (Faces::UBR, Cubie::corner(Faces::UBR).unwrap()),
                (Faces::DFR, Cubie::corner(Faces::DFR).unwrap()),
                (Faces::DFL, Cubie::corner(Faces::DFL).unwrap()),
                (Faces::DBL, Cubie::corner(Faces::DBL).unwrap()),
                (Faces::DBR, Cubie::corner(Faces::DBR).unwrap()),
                (Faces::UR, Cubie::edge(Faces::UR).unwrap()),
                (Faces::UF, Cubie::edge(Faces::UF).unwrap()),
                (Faces::UL, Cubie::edge(Faces::UL).unwrap()),
                (Faces::UB, Cubie::edge(Faces::UB).unwrap()),
                (Faces::DR, Cubie::edge(Faces::DR).unwrap()),
                (Faces::DF, Cubie::edge(Faces::DF).unwrap()),
                (Faces::DL, Cubie::edge(Faces::DL).unwrap()),
                (Faces::DB, Cubie::edge(Faces::DB).unwrap()),
                (Faces::FR, Cubie::edge(Faces::FR).unwrap()),
                (Faces::FL, Cubie::edge(Faces::FL).unwrap()),
                (Faces::BL, Cubie::edge(Faces::BL).unwrap()),
                (Faces::BR, Cubie::edge(Faces::BR).unwrap()),
            ])
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
