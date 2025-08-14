#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PieceLocation(usize);

impl PieceLocation {
    pub const UFR: PieceLocation = PieceLocation(0);
    pub const UFL: PieceLocation = PieceLocation(1);
    pub const UBR: PieceLocation = PieceLocation(2);
    pub const UBL: PieceLocation = PieceLocation(3);
    pub const DFR: PieceLocation = PieceLocation(4);
    pub const DFL: PieceLocation = PieceLocation(5);
    pub const DBR: PieceLocation = PieceLocation(6);
    pub const DBL: PieceLocation = PieceLocation(7);
    
    pub const UF: PieceLocation = PieceLocation(0);
    pub const UL: PieceLocation = PieceLocation(1);
    pub const UB: PieceLocation = PieceLocation(2);
    pub const UR: PieceLocation = PieceLocation(3);
    pub const DF: PieceLocation = PieceLocation(4);
    pub const DL: PieceLocation = PieceLocation(5);
    pub const DB: PieceLocation = PieceLocation(6);
    pub const DR: PieceLocation = PieceLocation(7);
    pub const FR: PieceLocation = PieceLocation(8);
    pub const FL: PieceLocation = PieceLocation(9);
    pub const BR: PieceLocation = PieceLocation(10);
    pub const BL: PieceLocation = PieceLocation(11);
}
