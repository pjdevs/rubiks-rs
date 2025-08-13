use crate::rubiks::faces::FaceMask;
use crate::rubiks::location::CubePieceLocation;
use crate::rubiks::twist::Twist;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CubePiece {
    original_location: CubePieceLocation,
    twist: Twist,
}

impl CubePiece {
    pub const UFR: CubePiece = CubePiece::corner(CubePieceLocation::new(FaceMask::UFR));
    pub const UFL: CubePiece = CubePiece::corner(CubePieceLocation::new(FaceMask::UFL));
    pub const UBL: CubePiece = CubePiece::corner(CubePieceLocation::new(FaceMask::UBL));
    pub const UBR: CubePiece = CubePiece::corner(CubePieceLocation::new(FaceMask::UBR));
    pub const DFR: CubePiece = CubePiece::corner(CubePieceLocation::new(FaceMask::DFR));
    pub const DFL: CubePiece = CubePiece::corner(CubePieceLocation::new(FaceMask::DFL));
    pub const DBL: CubePiece = CubePiece::corner(CubePieceLocation::new(FaceMask::DBL));
    pub const DBR: CubePiece = CubePiece::corner(CubePieceLocation::new(FaceMask::DBR));
    pub const UR: CubePiece = CubePiece::edge(CubePieceLocation::new(FaceMask::UR));
    pub const UF: CubePiece = CubePiece::edge(CubePieceLocation::new(FaceMask::UF));
    pub const UL: CubePiece = CubePiece::edge(CubePieceLocation::new(FaceMask::UL));
    pub const UB: CubePiece = CubePiece::edge(CubePieceLocation::new(FaceMask::UB));
    pub const DR: CubePiece = CubePiece::edge(CubePieceLocation::new(FaceMask::DR));
    pub const DF: CubePiece = CubePiece::edge(CubePieceLocation::new(FaceMask::DF));
    pub const DL: CubePiece = CubePiece::edge(CubePieceLocation::new(FaceMask::DL));
    pub const DB: CubePiece = CubePiece::edge(CubePieceLocation::new(FaceMask::DB));
    pub const FR: CubePiece = CubePiece::edge(CubePieceLocation::new(FaceMask::FR));
    pub const FL: CubePiece = CubePiece::edge(CubePieceLocation::new(FaceMask::FL));
    pub const BL: CubePiece = CubePiece::edge(CubePieceLocation::new(FaceMask::BL));
    pub const BR: CubePiece = CubePiece::edge(CubePieceLocation::new(FaceMask::BR));

    pub fn is_solved(&self, location: &CubePieceLocation) -> bool {
        self.original_location == *location && self.twist == Twist::SOLVED
    }

    pub const fn get_original_location(&self) -> CubePieceLocation {
        self.original_location
    }

    pub const fn get_twist(&self) -> Twist {
        self.twist
    }

    pub fn get_opposite_twist(&self) -> Twist {
        if self.is_corner() {
            self.twist.corner_opposite()
        } else {
            self.twist.edge_opposite()
        }
    }

    pub const fn is_corner(&self) -> bool {
        self.original_location.is_corner()
    }

    pub const fn is_edge(&self) -> bool {
        self.original_location.is_edge()
    }

    pub const fn twisted(&self, twist: Twist) -> CubePiece {
        CubePiece {
            original_location: self.original_location,
            twist: if self.is_corner() {
                self.twist.corner_add(&twist)
            } else {
                self.twist.edge_add(&twist)
            }
        }
    }

    const fn corner(location: CubePieceLocation) -> CubePiece {
        assert!(location.is_corner());

        CubePiece {
            original_location: location,
            twist: Twist::SOLVED
        }
    }

    const fn edge(location: CubePieceLocation) -> CubePiece {
        assert!(location.is_edge());

        CubePiece {
            original_location: location,
            twist: Twist::SOLVED
        }
    }
}