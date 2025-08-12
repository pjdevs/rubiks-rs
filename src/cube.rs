use std::iter::once;

use bitmask_enum::bitmask;
use enum_iterator::Sequence;
use enum_map::{Enum, EnumMap, enum_map};

pub trait CubeElement {
    fn get_face_mask(&self) -> Faces;
}

#[bitmask(u8)]
pub enum Faces {
    U,
    D,
    F,
    B,
    L,
    R,
    UFR = Self::U.or(Self::F).or(Self::R).bits,
    UFL = Self::U.or(Self::F).or(Self::L).bits,
    UBL = Self::U.or(Self::B).or(Self::L).bits,
    UBR = Self::U.or(Self::B).or(Self::R).bits,
    DFR = Self::D.or(Self::F).or(Self::R).bits,
    DFL = Self::D.or(Self::F).or(Self::L).bits,
    DBL = Self::D.or(Self::B).or(Self::L).bits,
    DBR = Self::D.or(Self::B).or(Self::R).bits,
    UB = Self::U.or(Self::B).bits,
    UR = Self::U.or(Self::R).bits,
    UF = Self::U.or(Self::F).bits,
    UL = Self::U.or(Self::L).bits,
    DB = Self::D.or(Self::B).bits,
    DR = Self::D.or(Self::R).bits,
    DF = Self::D.or(Self::L).bits,
    DL = Self::D.or(Self::F).bits,
    BR = Self::B.or(Self::R).bits,
    BL = Self::B.or(Self::L).bits,
    FR = Self::F.or(Self::R).bits,
    FL = Self::F.or(Self::L).bits,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CubeSticker {
    CornerSticker(CornerSticker),
    EdgeSticker(EdgeSticker),
}

impl CubeElement for CubeSticker {
    fn get_face_mask(&self) -> Faces {
        match self {
            CubeSticker::CornerSticker(corner_sticker) => corner_sticker.get_face_mask(),
            CubeSticker::EdgeSticker(edge_sticker) => edge_sticker.get_face_mask(),
        }
    }
}

impl CubeSticker {
    pub fn is_same_cubie(&self, other: &CubeSticker) -> bool {
        self.get_face_mask() == other.get_face_mask()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
pub enum CornerSticker {
    UFR, FRU, RFU,
    UFL, FLU, LUF,
    UBR, BRU, RUB,
    UBL, BLU, LUB,
    DFR, FRD, RDF,
    DFL, FLD, LDF,
    DBR, BRD, RDB,
    DBL, BLD, LDB,
}

impl CubeElement for CornerSticker {
    fn get_face_mask(&self) -> Faces {
        use CornerSticker::*;
        match self {
            UFR => Faces::UFR,
            FRU => Faces::UFR,
            RFU => Faces::UFR,
            UFL => Faces::UFL,
            FLU => Faces::UFL,
            LUF => Faces::UFL,
            UBR => Faces::UBR,
            BRU => Faces::UBR,
            RUB => Faces::UBR,
            UBL => Faces::UBL,
            BLU => Faces::UBL,
            LUB => Faces::UBL,
            DFR => Faces::DFR,
            FRD => Faces::DFR,
            RDF => Faces::DFR,
            DFL => Faces::DFL,
            FLD => Faces::DFL,
            LDF => Faces::DFL,
            DBR => Faces::DBR,
            BRD => Faces::DBR,
            RDB => Faces::DBR,
            DBL => Faces::DBL,
            BLD => Faces::DBL,
            LDB => Faces::DBL,
        }
    }
}

impl CornerSticker {
    pub fn is_same_cubie(&self, other: &CornerSticker) -> bool {
        self.get_face_mask() == other.get_face_mask()
    }

pub fn twisted(&self, twist: CornerOrientation) -> Self {
    use CornerSticker::*;
    use CornerOrientation::*;
    match self {
        UFR => match twist {
            Solved => UFR,
            Twisted120 => FRU,
            Twisted240 => RFU,
        },
        FRU => match twist {
            CornerOrientation::Solved => FRU,
            Twisted120 => RFU,
            Twisted240 => UFR,
        },
        RFU => match twist {
            Solved => RFU,
            Twisted120 => UFR,
            Twisted240 => FRU,
        },
        UFL => match twist {
            Solved => UFL,
            Twisted120 => FLU,
            Twisted240 => LUF,
        },
        FLU => match twist {
            Solved => FLU,
            Twisted120 => LUF,
            Twisted240 => UFL,
        },
        LUF => match twist {
            Solved => LUF,
            Twisted120 => UFL,
            Twisted240 => FLU,
        },
        UBR => match twist {
            Solved => UBR,
            Twisted120 => BRU,
            Twisted240 => RUB,
        },
        BRU => match twist {
            Solved => BRU,
            Twisted120 => RUB,
            Twisted240 => UBR,
        },
        RUB => match twist {
            Solved => RUB,
            Twisted120 => UBR,
            Twisted240 => BRU,
        },
        UBL => match twist {
            Solved => UBL,
            Twisted120 => BLU,
            Twisted240 => LUB,
        },
        BLU => match twist {
            Solved => BLU,
            Twisted120 => LUB,
            Twisted240 => UBL,
        },
        LUB => match twist {
            Solved => LUB,
            Twisted120 => UBL,
            Twisted240 => BLU,
        },
        DFR => match twist {
            Solved => DFR,
            Twisted120 => FRD,
            Twisted240 => RDF,
        },
        FRD => match twist {
            Solved => FRD,
            Twisted120 => RDF,
            Twisted240 => DFR,
        },
        RDF => match twist {
            Solved => RDF,
            Twisted120 => DFR,
            Twisted240 => FRD,
        },
        DFL => match twist {
            Solved => DFL,
            Twisted120 => FLD,
            Twisted240 => LDF,
        },
        FLD => match twist {
            Solved => FLD,
            Twisted120 => LDF,
            Twisted240 => DFL,
        },
        LDF => match twist {
            Solved => LDF,
            Twisted120 => DFL,
            Twisted240 => FLD,
        },
        DBR => match twist {
            Solved => DBR,
            Twisted120 => BRD,
            Twisted240 => RDB,
        },
        BRD => match twist {
            Solved => BRD,
            Twisted120 => RDB,
            Twisted240 => DBR,
        },
        RDB => match twist {
            Solved => RDB,
            Twisted120 => DBR,
            Twisted240 => BRD,
        },
        DBL => match twist {
            Solved => DBL,
            Twisted120 => BLD,
            Twisted240 => LDB,
        },
        BLD => match twist {
            Solved => BLD,
            Twisted120 => LDB,
            Twisted240 => DBL,
        },
        LDB => match twist {
            Solved => LDB,
            Twisted120 => DBL,
            Twisted240 => BLD,
        },
    }
}
}

impl TryFrom<Faces> for CornerSticker {
    type Error = ();

    fn try_from(value: Faces) -> Result<Self, Self::Error> {
        use CornerSticker::*;
        match value {
            Faces::UFR => Ok(UFR),
            Faces::UFL => Ok(UFL),
            Faces::UBR => Ok(UBR),
            Faces::UBL => Ok(UBL),
            Faces::DFR => Ok(DFR),
            Faces::DFL => Ok(DFL),
            Faces::DBR => Ok(DBR),
            Faces::DBL => Ok(DBL),
            _ => Err(())
        }
    }
}

impl Into<CubeSticker> for EdgeSticker {
    fn into(self) -> CubeSticker {
        CubeSticker::EdgeSticker(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Sequence)]
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

impl TryFrom<Faces> for EdgeSticker {
    type Error = ();

    fn try_from(value: Faces) -> Result<Self, Self::Error> {
        use EdgeSticker::*;
        match value {
            Faces::UF => Ok(UF),
            Faces::UL => Ok(UL),
            Faces::UB => Ok(UB),
            Faces::UR => Ok(UR),
            Faces::DF => Ok(DF),
            Faces::DL => Ok(DL),
            Faces::DB => Ok(DB),
            Faces::DR => Ok(DR),
            Faces::FR => Ok(FR),
            Faces::FL => Ok(FL),
            Faces::BR => Ok(BR),
            Faces::BL => Ok(BL),
            _ => Err(())
        }
    }
}

impl CubeElement for EdgeSticker {
    fn get_face_mask(&self) -> Faces {
        use EdgeSticker::*;
        match self {
            UF => Faces::UF,
            FU => Faces::UF,
            UL => Faces::UL,
            LU => Faces::UL,
            UB => Faces::UB,
            BU => Faces::UB,
            UR => Faces::UR,
            RU => Faces::UR,
            DF => Faces::DF,
            FD => Faces::DF,
            DL => Faces::DL,
            LD => Faces::DL,
            DB => Faces::DB,
            BD => Faces::DB,
            DR => Faces::DR,
            RD => Faces::DR,
            FR => Faces::FR,
            RF => Faces::FR,
            FL => Faces::FL,
            LF => Faces::FL,
            BR => Faces::BR,
            RB => Faces::BR,
            BL => Faces::BL,
            LB => Faces::BL,
        }
    }
}

impl EdgeSticker {
    pub fn is_same_cubie(&self, other: &EdgeSticker) -> bool {
        self.get_face_mask() == other.get_face_mask()
    }

    pub fn inverted(&self) -> Self {
        use EdgeSticker::*;
        match self {
            UF => FU,
            FU => UF,
            UL => LU,
            LU => UL,
            UB => BU,
            BU => UB,
            UR => RU,
            RU => UR,
            DF => FD,
            FD => DF,
            DL => LD,
            LD => DL,
            DB => BD,
            BD => DB,
            DR => RD,
            RD => DR,
            FR => RF,
            RF => FR,
            FL => LF,
            LF => FL,
            BR => RB,
            RB => BR,
            BL => LB,
            LB => BL,
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
            Self::Solved => Self::Flipped,
            Self::Flipped => Self::Solved,
        }
    }
}

#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Corner {
    UFR,
    UFL,
    UBR,
    UBL,
    DFR,
    DFL,
    DBL,
    DBR,
}

impl CubeElement for Corner {
    fn get_face_mask(&self) -> Faces {
        use Corner::*;
        match self {
            UFR => Faces::UFR,
            UFL => Faces::UFL,
            UBL => Faces::UBL,
            UBR => Faces::UBR,
            DFR => Faces::DFR,
            DFL => Faces::DFL,
            DBL => Faces::DBL,
            DBR => Faces::DBR,
        }
    }
}

#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Edge {
    UB,
    UR,
    UF,
    UL,
    DB,
    DR,
    DF,
    DL,
    BL,
    BR,
    FR,
    FL,
}

impl CubeElement for Edge {
    fn get_face_mask(&self) -> Faces {
        use Edge::*;
        match self {
            UB => Faces::UB,
            UR => Faces::UR,
            UF => Faces::UF,
            UL => Faces::UL,
            DB => Faces::DB,
            DR => Faces::DR,
            DF => Faces::DF,
            DL => Faces::DL,
            BR => Faces::BR,
            BL => Faces::BL,
            FR => Faces::FR,
            FL => Faces::FL,
        }
    }
}

#[derive(Enum, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CubeLocation {
    CornerLocation(Corner),
    EdgeLocation(Edge)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CornerCubie {
    piece: Corner,
    orientation: CornerOrientation
}

impl CubeElement for CornerCubie {
    fn get_face_mask(&self) -> Faces {
        self.piece.get_face_mask()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EdgeCubie {
    piece: Edge,
    orientation: EdgeOrientation
}

impl CubeElement for EdgeCubie {
    fn get_face_mask(&self) -> Faces {
        self.piece.get_face_mask()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cubie {
    CornerCubie(CornerCubie),
    EdgeCubie(EdgeCubie)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeMove {
    U, R, Up, Rp
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cube {
    corners: EnumMap<Corner, CornerCubie>,
    edges: EnumMap<Edge, EdgeCubie>,
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

    pub fn get_sticker_at(&self, sticker_location: CubeSticker) -> CubeSticker {
        let piece_location = Cube::get_location_from(sticker_location)
            .expect("Sticker should have a valid location");
        
        let cubie = match piece_location {
            CubeLocation::CornerLocation(corner) => Cubie::CornerCubie(self.corners[corner]),  
            CubeLocation::EdgeLocation(edge) => Cubie::EdgeCubie(self.edges[edge]),
        };

        Cube::sticker_inside(&cubie)
    }

    // TODO take into account the request sticker!
    fn sticker_inside(cubie: &Cubie) -> CubeSticker {
        match cubie {
            Cubie::CornerCubie(corner_cubie) => {
                let mask = corner_cubie.get_face_mask();
                let sticker = CornerSticker::try_from(mask).expect("Corner cubie mask should be a corner sticker");
                let oriented_sticker = sticker.twisted(corner_cubie.orientation);
                CubeSticker::CornerSticker(oriented_sticker)
            },
            Cubie::EdgeCubie(edge_cubie) => {
                let mask = edge_cubie.get_face_mask();
                let sticker = EdgeSticker::try_from(mask).expect("Edhe cubie mask should be an edge sticker");
                let oriented_sticker = match edge_cubie.orientation {
                    EdgeOrientation::Solved => sticker,
                    EdgeOrientation::Flipped => sticker.inverted(),
                };
                CubeSticker::EdgeSticker(oriented_sticker)
            },
        }
    }

    fn get_location_from<TElement: CubeElement>(element: TElement) -> Option<CubeLocation> {
        match element.get_face_mask() {
            Faces::UFR => Some(CubeLocation::CornerLocation(Corner::UFR)),
            Faces::UFL => Some(CubeLocation::CornerLocation(Corner::UFL)),
            Faces::UBL => Some(CubeLocation::CornerLocation(Corner::UBL)),
            Faces::UBR => Some(CubeLocation::CornerLocation(Corner::UBR)),
            Faces::DFR => Some(CubeLocation::CornerLocation(Corner::DFR)),
            Faces::DFL => Some(CubeLocation::CornerLocation(Corner::DFL)),
            Faces::DBL => Some(CubeLocation::CornerLocation(Corner::UFL)),
            Faces::DBR => Some(CubeLocation::CornerLocation(Corner::DBR)),
            Faces::UB => Some(CubeLocation::EdgeLocation(Edge::UB)),
            Faces::UR => Some(CubeLocation::EdgeLocation(Edge::UR)),
            Faces::UF => Some(CubeLocation::EdgeLocation(Edge::UF)),
            Faces::UL => Some(CubeLocation::EdgeLocation(Edge::UL)),
            Faces::DB => Some(CubeLocation::EdgeLocation(Edge::DB)),
            Faces::DR => Some(CubeLocation::EdgeLocation(Edge::DR)),
            Faces::DF => Some(CubeLocation::EdgeLocation(Edge::DF)),
            Faces::DL => Some(CubeLocation::EdgeLocation(Edge::DL)),
            Faces::BR => Some(CubeLocation::EdgeLocation(Edge::BR)),
            Faces::BL => Some(CubeLocation::EdgeLocation(Edge::BL)),
            Faces::FR => Some(CubeLocation::EdgeLocation(Edge::FR)),
            Faces::FL => Some(CubeLocation::EdgeLocation(Edge::FL)),
            _ => None
        }
    }
}
