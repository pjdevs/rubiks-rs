use crate::faces::{Face, FaceMask};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CubePieceLocation(FaceMask);

impl CubePieceLocation {
    pub const UB: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::U.or(FaceMask::B));
    pub const UR: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::U.or(FaceMask::R));
    pub const UF: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::U.or(FaceMask::F));
    pub const UL: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::U.or(FaceMask::L));
    pub const DB: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::D.or(FaceMask::B));
    pub const DR: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::D.or(FaceMask::R));
    pub const DF: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::D.or(FaceMask::L));
    pub const DL: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::D.or(FaceMask::F));
    pub const BR: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::B.or(FaceMask::R));
    pub const BL: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::B.or(FaceMask::L));
    pub const FR: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::F.or(FaceMask::R));
    pub const FL: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::F.or(FaceMask::L));
    pub const URF: CubePieceLocation =
        CubePieceLocation::from_mask(FaceMask::U.or(FaceMask::F).or(FaceMask::R));
    pub const UFL: CubePieceLocation =
        CubePieceLocation::from_mask(FaceMask::U.or(FaceMask::F).or(FaceMask::L));
    pub const ULB: CubePieceLocation =
        CubePieceLocation::from_mask(FaceMask::U.or(FaceMask::B).or(FaceMask::L));
    pub const UBR: CubePieceLocation =
        CubePieceLocation::from_mask(FaceMask::U.or(FaceMask::B).or(FaceMask::R));
    pub const DFR: CubePieceLocation =
        CubePieceLocation::from_mask(FaceMask::D.or(FaceMask::F).or(FaceMask::R));
    pub const DLF: CubePieceLocation =
        CubePieceLocation::from_mask(FaceMask::D.or(FaceMask::F).or(FaceMask::L));
    pub const DBL: CubePieceLocation =
        CubePieceLocation::from_mask(FaceMask::D.or(FaceMask::B).or(FaceMask::L));
    pub const DRB: CubePieceLocation =
        CubePieceLocation::from_mask(FaceMask::D.or(FaceMask::B).or(FaceMask::R));

    const fn from_mask(mask: FaceMask) -> CubePieceLocation {
        assert!(mask.is_corner() || mask.is_edge());
        Self(mask)
    }

    pub const fn mask(self) -> FaceMask {
        self.0
    }

    pub const fn is_corner(&self) -> bool {
        self.0.is_corner()
    }

    pub const fn is_edge(&self) -> bool {
        self.0.is_edge()
    }

    pub fn get_faces(&self) -> Vec<Face> {
        use Face::*;
        match *self {
            Self::UB => vec![U, B],
            Self::UR => vec![U, R],
            Self::UF => vec![U, F],
            Self::UL => vec![U, L],
            Self::DB => vec![D, B],
            Self::DR => vec![D, R],
            Self::DF => vec![D, L],
            Self::DL => vec![D, F],
            Self::BR => vec![B, R],
            Self::BL => vec![B, L],
            Self::FR => vec![F, R],
            Self::FL => vec![F, L],
            Self::URF => vec![U, R, F],
            Self::UFL => vec![U, F, L],
            Self::ULB => vec![U, L, B],
            Self::UBR => vec![U, B, R],
            Self::DFR => vec![D, F, R],
            Self::DLF => vec![D, L, F],
            Self::DBL => vec![D, B, L],
            Self::DRB => vec![D, R, B],
            _ => vec![]
        }
    }
}
