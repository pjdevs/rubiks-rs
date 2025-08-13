use crate::rubiks::faces::FaceMask;

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
    pub const UFR: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::U.or(FaceMask::F).or(FaceMask::R));
    pub const UFL: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::U.or(FaceMask::F).or(FaceMask::L));
    pub const UBL: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::U.or(FaceMask::B).or(FaceMask::L));
    pub const UBR: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::U.or(FaceMask::B).or(FaceMask::R));
    pub const DFR: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::D.or(FaceMask::F).or(FaceMask::R));
    pub const DFL: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::D.or(FaceMask::F).or(FaceMask::L));
    pub const DBL: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::D.or(FaceMask::B).or(FaceMask::L));
    pub const DBR: CubePieceLocation = CubePieceLocation::from_mask(FaceMask::D.or(FaceMask::B).or(FaceMask::R));

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
}
