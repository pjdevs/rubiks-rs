use crate::{faces::{CORNER_FACES, EDGE_FACES}, location::PieceLocation, piece::{CornerTwist, EdgeTwist, Twist}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StickerLocation<const FACES: usize> {
    pub location: PieceLocation,
    pub twist: Twist<FACES>,
}

pub type CornerSticker = StickerLocation<CORNER_FACES>;
pub type EdgeSticker = StickerLocation<EDGE_FACES>;

impl CornerSticker {
    pub const UFR: CornerSticker = CornerSticker { location: PieceLocation::UFR, twist: CornerTwist::solved() };
    pub const UFL: CornerSticker = CornerSticker { location: PieceLocation::UFL, twist: CornerTwist::solved() };
    pub const UBR: CornerSticker = CornerSticker { location: PieceLocation::UBR, twist: CornerTwist::solved() };
    pub const UBL: CornerSticker = CornerSticker { location: PieceLocation::UBL, twist: CornerTwist::solved() };
    pub const DFR: CornerSticker = CornerSticker { location: PieceLocation::DFR, twist: CornerTwist::solved() };
    pub const DFL: CornerSticker = CornerSticker { location: PieceLocation::DFL, twist: CornerTwist::solved() };
    pub const DBR: CornerSticker = CornerSticker { location: PieceLocation::DBR, twist: CornerTwist::solved() };
    pub const DBL: CornerSticker = CornerSticker { location: PieceLocation::DBL, twist: CornerTwist::solved() };
}

impl EdgeSticker {
    pub const UF: EdgeSticker = EdgeSticker { location: PieceLocation::UF, twist: EdgeTwist::solved() };
    pub const UL: EdgeSticker = EdgeSticker { location: PieceLocation::UL, twist: EdgeTwist::solved() };
    pub const UB: EdgeSticker = EdgeSticker { location: PieceLocation::UB, twist: EdgeTwist::solved() };
    pub const UR: EdgeSticker = EdgeSticker { location: PieceLocation::UR, twist: EdgeTwist::solved() };
    pub const DF: EdgeSticker = EdgeSticker { location: PieceLocation::DF, twist: EdgeTwist::solved() };
    pub const DL: EdgeSticker = EdgeSticker { location: PieceLocation::DL, twist: EdgeTwist::solved() };
    pub const DB: EdgeSticker = EdgeSticker { location: PieceLocation::DB, twist: EdgeTwist::solved() };
    pub const DR: EdgeSticker = EdgeSticker { location: PieceLocation::DR, twist: EdgeTwist::solved() };
    pub const FR: EdgeSticker = EdgeSticker { location: PieceLocation::FR, twist: EdgeTwist::solved() };
    pub const FL: EdgeSticker = EdgeSticker { location: PieceLocation::FL, twist: EdgeTwist::solved() };
    pub const BR: EdgeSticker = EdgeSticker { location: PieceLocation::BR, twist: EdgeTwist::solved() };
    pub const BL: EdgeSticker = EdgeSticker { location: PieceLocation::BL, twist: EdgeTwist::solved() };
}