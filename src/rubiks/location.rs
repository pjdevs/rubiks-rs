use crate::rubiks::faces::FaceMask;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct CubePieceLocation(FaceMask);

impl CubePieceLocation {
    pub const fn new(value: FaceMask) -> Self {
        Self(value)
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
