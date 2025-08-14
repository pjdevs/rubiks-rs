use crate::{faces::Face, twist::Twist};

#[derive(Clone, Copy, Debug)]
pub struct Piece<const FACES: usize> {
    stickers: [Face; FACES],
}

impl<const FACES: usize> Piece<FACES> {
    const fn new(stickers: [Face; FACES]) -> Piece<FACES> {
        Self { stickers }
    }

    pub const fn is_corner() -> bool {
        FACES == 3
    }

    pub const fn is_edge() -> bool {
        FACES == 2
    }
}

pub struct PieceState<const FACES: usize> {
    pub piece: Piece<FACES>,
    pub twist: Twist,
}

pub type Corner = Piece<3>;
pub type Edge = Piece<2>;
pub type CornerState = PieceState<3>;
pub type EdgeState = PieceState<2>;

impl Corner {
        pub const UFR: Corner = Corner::new([Face::U, Face::F, Face::R]); 
        pub const UFL: Corner = Corner::new([Face::U, Face::L, Face::F]); 
        pub const UBR: Corner = Corner::new([Face::U, Face::B, Face::R]); 
        pub const UBL: Corner = Corner::new([Face::U, Face::L, Face::B]); 
        pub const DFR: Corner = Corner::new([Face::D, Face::F, Face::R]); 
        pub const DFL: Corner = Corner::new([Face::D, Face::L, Face::F]); 
        pub const DBR: Corner = Corner::new([Face::D, Face::B, Face::R]); 
        pub const DLB: Corner = Corner::new([Face::D, Face::L, Face::B]); 
}

impl Edge {
    pub const UF: Edge = Piece::new([Face::U, Face::F]);
    pub const UL: Edge = Piece::new([Face::U, Face::L]);
    pub const UB: Edge = Piece::new([Face::U, Face::B]);
    pub const UR: Edge = Piece::new([Face::U, Face::R]);
    pub const DF: Edge = Piece::new([Face::D, Face::F]);
    pub const DL: Edge = Piece::new([Face::D, Face::L]);
    pub const DB: Edge = Piece::new([Face::D, Face::B]);
    pub const DR: Edge = Piece::new([Face::D, Face::R]);
    pub const FR: Edge = Piece::new([Face::F, Face::R]);
    pub const FL: Edge = Piece::new([Face::F, Face::L]);
    pub const BR: Edge = Piece::new([Face::B, Face::R]);
    pub const BL: Edge = Piece::new([Face::B, Face::L]);
}