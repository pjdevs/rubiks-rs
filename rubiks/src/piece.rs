use crate::location::CubePieceLocation;
use crate::twist::Twist;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CubePiece {
    original_location: CubePieceLocation,
    twist: Twist,
}

impl CubePiece {
    // ? Could only use location and from_location but keep for helper ?
    pub const UFR: CubePiece = CubePiece::corner(CubePieceLocation::URF);
    pub const UFL: CubePiece = CubePiece::corner(CubePieceLocation::UFL);
    pub const UBL: CubePiece = CubePiece::corner(CubePieceLocation::ULB);
    pub const UBR: CubePiece = CubePiece::corner(CubePieceLocation::UBR);
    pub const DFR: CubePiece = CubePiece::corner(CubePieceLocation::DFR);
    pub const DFL: CubePiece = CubePiece::corner(CubePieceLocation::DLF);
    pub const DBL: CubePiece = CubePiece::corner(CubePieceLocation::DBL);
    pub const DBR: CubePiece = CubePiece::corner(CubePieceLocation::DRB);
    pub const UR: CubePiece = CubePiece::edge(CubePieceLocation::UR);
    pub const UF: CubePiece = CubePiece::edge(CubePieceLocation::UF);
    pub const UL: CubePiece = CubePiece::edge(CubePieceLocation::UL);
    pub const UB: CubePiece = CubePiece::edge(CubePieceLocation::UB);
    pub const DR: CubePiece = CubePiece::edge(CubePieceLocation::DR);
    pub const DF: CubePiece = CubePiece::edge(CubePieceLocation::DF);
    pub const DL: CubePiece = CubePiece::edge(CubePieceLocation::DL);
    pub const DB: CubePiece = CubePiece::edge(CubePieceLocation::DB);
    pub const FR: CubePiece = CubePiece::edge(CubePieceLocation::FR);
    pub const FL: CubePiece = CubePiece::edge(CubePieceLocation::FL);
    pub const BL: CubePiece = CubePiece::edge(CubePieceLocation::BL);
    pub const BR: CubePiece = CubePiece::edge(CubePieceLocation::BR);

    pub const fn from_location(location: CubePieceLocation) -> Self {
        if location.is_corner() {
            Self::corner(location)
        } else {
            Self::edge(location)
        }
    }

    pub fn is_solved(&self, location: &CubePieceLocation) -> bool {
        self.original_location == *location && self.twist == Twist::SOLVED
    }

    pub const fn get_original_location(&self) -> CubePieceLocation {
        self.original_location
    }

    pub const fn get_twist(&self) -> Twist {
        self.twist
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
                self.twist.corner_add(twist)
            } else {
                self.twist.edge_add(twist)
            },
        }
    }

    const fn corner(location: CubePieceLocation) -> CubePiece {
        assert!(location.is_corner());

        CubePiece {
            original_location: location,
            twist: Twist::SOLVED,
        }
    }

    const fn edge(location: CubePieceLocation) -> CubePiece {
        assert!(location.is_edge());

        CubePiece {
            original_location: location,
            twist: Twist::SOLVED,
        }
    }
}
