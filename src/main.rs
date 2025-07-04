use std::iter::once;

use enum_map::{Enum, EnumMap, enum_map};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CubeSticker {
    // Corner stickers
    UFR, URF, RFU,
    UFL, ULF, FLU,
    UBR, URB, BRU,
    UBL, ULB, BLU,
    DFR, DRF, FRD,
    DFL, DLF, FLD,
    DBR, DRB, BRD,
    DBL, DLB, BLD,

    // Edge stickers
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

    pub fn get_sticker_color(&self, _sticker: CubeSticker) {
        todo!("Implement colors");
    }
}

fn main() {
    use CubeMove::*;

    let mut cube = Cube::solved();
    cube.apply_moves(&vec![
        R, R, R, R,
        U, U, U, U,
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up,
        R, U, Rp, Up,
    ]);

    println!("{:?}", cube == Cube::solved());
}
