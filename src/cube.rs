use std::iter::once;

use enum_map::{Enum, EnumMap, enum_map};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Faces {
    U = 1 << 0,
    D = 1 << 1,
    F = 1 << 2,
    B = 1 << 3,
    L = 1 << 4,
    R = 1 << 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CubeSticker {
    CornerSticker(CornerSticker),
    EdgeSticker(EdgeSticker),
}

impl CubeSticker {
    pub fn is_same_cubie(&self, other: &CubeSticker) -> bool {
        self.get_face_mask() == other.get_face_mask()
    }

    pub fn get_face_mask(&self) -> u8 {
        match self {
            CubeSticker::CornerSticker(corner_sticker) => corner_sticker.get_face_mask(),
            CubeSticker::EdgeSticker(edge_sticker) => edge_sticker.get_face_mask(),
        }
    }
}

impl TryInto<CornerSticker> for CubeSticker {
    type Error = ();

    fn try_into(self) -> Result<CornerSticker, Self::Error> {
        match self {
            CubeSticker::CornerSticker(corner_sticker) => Result::Ok(corner_sticker),
            CubeSticker::EdgeSticker(_) => Result::Err(()),
        }
    }
}

impl TryInto<EdgeSticker> for CubeSticker {
    type Error = ();

    fn try_into(self) -> Result<EdgeSticker, Self::Error> {
        match self {
            CubeSticker::CornerSticker(_) => Result::Err(()),
            CubeSticker::EdgeSticker(edge_sticker) => Result::Ok(edge_sticker),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CornerSticker {
    UFR, FRU, RFU,
    UFL, FLU, LFU,
    UBR, BRU, RBU,
    UBL, BLU, LBU,
    DFR, FRD, RFD,
    DFL, FLD, LFD,
    DBR, BRD, RBD,
    DBL, BLD, LBD,
}

impl CornerSticker {
    pub fn is_same_cubie(&self, other: &CornerSticker) -> bool {
        self.get_face_mask() == other.get_face_mask()
    }

    pub fn get_face_mask(&self) -> u8 {
        match self {
            CornerSticker::UFR => Faces::U as u8 | Faces::F as u8 | Faces::R as u8,
            CornerSticker::FRU => Faces::U as u8 | Faces::F as u8 | Faces::R as u8,
            CornerSticker::RFU => Faces::U as u8 | Faces::F as u8 | Faces::R as u8,
            CornerSticker::UFL => Faces::U as u8 | Faces::F as u8 | Faces::L as u8,
            CornerSticker::FLU => Faces::U as u8 | Faces::F as u8 | Faces::L as u8,
            CornerSticker::LFU => Faces::U as u8 | Faces::F as u8 | Faces::L as u8,
            CornerSticker::UBR => Faces::U as u8 | Faces::B as u8 | Faces::R as u8,
            CornerSticker::BRU => Faces::U as u8 | Faces::B as u8 | Faces::R as u8,
            CornerSticker::RBU => Faces::U as u8 | Faces::B as u8 | Faces::R as u8,
            CornerSticker::UBL => Faces::U as u8 | Faces::B as u8 | Faces::L as u8,
            CornerSticker::BLU => Faces::U as u8 | Faces::B as u8 | Faces::L as u8,
            CornerSticker::LBU => Faces::U as u8 | Faces::B as u8 | Faces::L as u8,
            CornerSticker::DFR => Faces::D as u8 | Faces::F as u8 | Faces::R as u8,
            CornerSticker::FRD => Faces::D as u8 | Faces::F as u8 | Faces::R as u8,
            CornerSticker::RFD => Faces::D as u8 | Faces::F as u8 | Faces::R as u8,
            CornerSticker::DFL => Faces::D as u8 | Faces::F as u8 | Faces::L as u8,
            CornerSticker::FLD => Faces::D as u8 | Faces::F as u8 | Faces::L as u8,
            CornerSticker::LFD => Faces::D as u8 | Faces::F as u8 | Faces::L as u8,
            CornerSticker::DBR => Faces::D as u8 | Faces::B as u8 | Faces::R as u8,
            CornerSticker::BRD => Faces::D as u8 | Faces::B as u8 | Faces::R as u8,
            CornerSticker::RBD => Faces::D as u8 | Faces::B as u8 | Faces::R as u8,
            CornerSticker::DBL => Faces::D as u8 | Faces::B as u8 | Faces::L as u8,
            CornerSticker::BLD => Faces::D as u8 | Faces::B as u8 | Faces::L as u8,
            CornerSticker::LBD => Faces::D as u8 | Faces::B as u8 | Faces::L as u8,
        }
    }
}

impl Into<CubeSticker> for EdgeSticker {
    fn into(self) -> CubeSticker {
        CubeSticker::EdgeSticker(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeSticker {
    UF, FU,
    UL, LU,
    UB, BU,
    UR, RU,
    DF, FD,
    DL, LD,
    DB, BD,
    DR, RD,
    FR, RF,
    FL, LF,
    BR, RB,
    BL, LB,
}

impl EdgeSticker {
    pub fn is_same_cubie(&self, other: &EdgeSticker) -> bool {
        self.get_face_mask() == other.get_face_mask()
    }

    fn get_face_mask(&self) -> u8 {
        match self {
            EdgeSticker::UF => Faces::U as u8 | Faces::F as u8,
            EdgeSticker::FU => Faces::U as u8 | Faces::F as u8,
            EdgeSticker::UL => Faces::U as u8 | Faces::L as u8,
            EdgeSticker::LU => Faces::U as u8 | Faces::L as u8,
            EdgeSticker::UB => Faces::U as u8 | Faces::B as u8,
            EdgeSticker::BU => Faces::U as u8 | Faces::B as u8,
            EdgeSticker::UR => Faces::U as u8 | Faces::R as u8,
            EdgeSticker::RU => Faces::U as u8 | Faces::R as u8,
            EdgeSticker::DF => Faces::D as u8 | Faces::F as u8,
            EdgeSticker::FD => Faces::D as u8 | Faces::F as u8,
            EdgeSticker::DL => Faces::D as u8 | Faces::L as u8,
            EdgeSticker::LD => Faces::D as u8 | Faces::L as u8,
            EdgeSticker::DB => Faces::D as u8 | Faces::B as u8,
            EdgeSticker::BD => Faces::D as u8 | Faces::B as u8,
            EdgeSticker::DR => Faces::D as u8 | Faces::R as u8,
            EdgeSticker::RD => Faces::D as u8 | Faces::R as u8,
            EdgeSticker::FR => Faces::F as u8 | Faces::R as u8,
            EdgeSticker::RF => Faces::F as u8 | Faces::R as u8,
            EdgeSticker::FL => Faces::F as u8 | Faces::L as u8,
            EdgeSticker::LF => Faces::F as u8 | Faces::L as u8,
            EdgeSticker::BR => Faces::B as u8 | Faces::R as u8,
            EdgeSticker::RB => Faces::B as u8 | Faces::R as u8,
            EdgeSticker::BL => Faces::B as u8 | Faces::L as u8,
            EdgeSticker::LB => Faces::B as u8 | Faces::L as u8,
        }
    }
}

impl Into<CubeSticker> for CornerSticker {
    fn into(self) -> CubeSticker {
        CubeSticker::CornerSticker(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CornerOrientation {
    Solved = 0,
    Twisted120 = 1,
    Twisted240 = 2,
}

impl CornerOrientation {
    pub fn twisted(&self, twist: CornerOrientation) -> CornerOrientation {
        CornerOrientation::from((*self as u8  + twist as u8) % 3u8)
    }
}

impl From<u8> for CornerOrientation {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Solved,
            1 => Self::Twisted120,
            2 => Self::Twisted240,
            _ => Self::Solved
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EdgeOrientation {
    Solved,
    Flipped,
}

impl EdgeOrientation {
    pub fn flipped(&self) -> EdgeOrientation {
        match self {
            EdgeOrientation::Solved => Self::Flipped,
            EdgeOrientation::Flipped => Self::Solved,
        }
    }
}

#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Corner {
    UFR, UFL, UBL, UBR, DFR, DFL, DBL, DBR,
}

#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Edge {
    UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CornerCubie {
    piece: Corner,
    orientation: CornerOrientation
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EdgeCubie {
    piece: Edge,
    orientation: EdgeOrientation
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cube {
    corners: EnumMap<Corner, CornerCubie>,
    edges: EnumMap<Edge, EdgeCubie>,
}

pub enum CubeMove {
    U, R, Up, Rp
}

impl Cube {
    pub fn apply_moves(&mut self, moves: &Vec<CubeMove>) {
        for cube_move in moves {
            self.apply_move(cube_move);
        }
    }

    fn apply_move(&mut self, cube_move: &CubeMove) {
        use Corner::*;
        use Edge::*;
        use CornerOrientation::*;

        match cube_move {
            CubeMove::U => {
                self.cycle_corners(
                    &[UFL, UBL, UBR, UFR],
                    &[Solved, Solved, Solved, Solved]
                );
                self.cycle_edges(&[UF, UL, UB, UR], false);
            },
            CubeMove::R => {
                self.cycle_corners(
                    &[UFR, UBR, DBR, DFR],
                    &[Twisted120, Twisted240, Twisted120, Twisted240]
                );
                self.cycle_edges(&[UR, BR, DR, FR], true);
            },
            CubeMove::Up => {
                self.cycle_corners(
                    &[UFL, UFR, UBR, UBL],
                    &[Solved, Solved, Solved, Solved]
                );
                self.cycle_edges(&[UF, UR, UB, UL], false);
            },
            CubeMove::Rp => {
                self.cycle_corners(
                    &[UFR, DFR, DBR, UBR],
                    &[Twisted240, Twisted120, Twisted240, Twisted120]
                );
                self.cycle_edges(&[UR, FR, DR, BR], true);
            },
        }
    }

    fn cycle_corners(&mut self, corners: &[Corner], twists: &[CornerOrientation]) {
        let first_position = corners.first().expect("Corner cycles should be at least of size 1");
        let mut last_cubie = self.corners[*first_position];
        
        for (target_position, twist) in corners.iter().skip(1).chain(once(first_position)).zip(twists) {
            let target_cubie = &mut self.corners[*target_position];
            
            // save which one was here before permutation
            let cubie_to_send = last_cubie;
            last_cubie = *target_cubie;

            // do the permutation
            *target_cubie = CornerCubie {
                piece: cubie_to_send.piece,
                orientation: cubie_to_send.orientation.twisted(*twist)
            };
        }
    }

    fn cycle_edges(&mut self, edges: &[Edge], flip: bool) {
        let first_position = edges.first().expect("Edge cycles should be at least of size 1");
        let mut last_cubie = self.edges[*first_position];
        
        for target_position in edges.iter().skip(1).chain(once(first_position)) {
            let target_cubie = &mut self.edges[*target_position];
            
            // save which one was here before permutation
            let cubie_to_send = last_cubie;
            last_cubie = *target_cubie;

            // do the permutation
            *target_cubie = EdgeCubie {
                piece: cubie_to_send.piece,
                orientation: if flip { cubie_to_send.orientation.flipped() } else { cubie_to_send.orientation } 
            };
        }
    }

    pub fn solved() -> Cube {
        use Corner::*;
        use Edge::*;
        Cube {
            corners: enum_map! {
                UFR => CornerCubie { piece: UFR, orientation: CornerOrientation::Solved },
                UFL => CornerCubie { piece: UFL, orientation: CornerOrientation::Solved },
                UBL => CornerCubie { piece: UBL, orientation: CornerOrientation::Solved },
                UBR => CornerCubie { piece: UBR, orientation: CornerOrientation::Solved },
                DFR => CornerCubie { piece: DFR, orientation: CornerOrientation::Solved },
                DFL => CornerCubie { piece: DFL, orientation: CornerOrientation::Solved },
                DBL => CornerCubie { piece: DBL, orientation: CornerOrientation::Solved },
                DBR => CornerCubie { piece: DBR, orientation: CornerOrientation::Solved },
            },
            edges: enum_map! {
                UR => EdgeCubie { piece: UR, orientation: EdgeOrientation::Solved },
                UF => EdgeCubie { piece: UF, orientation: EdgeOrientation::Solved },
                UL => EdgeCubie { piece: UL, orientation: EdgeOrientation::Solved },
                UB => EdgeCubie { piece: UB, orientation: EdgeOrientation::Solved },
                DR => EdgeCubie { piece: DR, orientation: EdgeOrientation::Solved },
                DF => EdgeCubie { piece: DF, orientation: EdgeOrientation::Solved },
                DL => EdgeCubie { piece: DL, orientation: EdgeOrientation::Solved },
                DB => EdgeCubie { piece: DB, orientation: EdgeOrientation::Solved },
                FR => EdgeCubie { piece: FR, orientation: EdgeOrientation::Solved },
                FL => EdgeCubie { piece: FL, orientation: EdgeOrientation::Solved },
                BL => EdgeCubie { piece: BL, orientation: EdgeOrientation::Solved },
                BR => EdgeCubie { piece: BR, orientation: EdgeOrientation::Solved },
            },
        }
    }

    pub fn get_sticker_color(&self, _sticker: CubeSticker) -> CubeSticker {
        todo!("Implement colors");
    }
}
