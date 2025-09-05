use crate::cube_constants::*;
use bitmask_enum::bitmask;
use core::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Face {
    U,
    D,
    F,
    B,
    L,
    R,
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[bitmask(u8)]
pub enum FaceMask {
    U,
    D,
    F,
    B,
    L,
    R,
}

impl FaceMask {
    pub const fn is_corner(self) -> bool {
        self.bits().count_ones() == STICKERS_ON_CORNERS as u32
    }

    pub const fn is_edge(self) -> bool {
        self.bits().count_ones() == STICKERS_ON_EDGES as u32
    }
}
