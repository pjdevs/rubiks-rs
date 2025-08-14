use core::fmt;
use bitmask_enum::bitmask;
use crate::cube_constants::*;

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

    pub fn faces(&self) -> Vec<Face> {
        let mut faces = Vec::new();

        if self.contains(Self::U) {
            faces.push(Face::U);
        } else if self.contains(Self::D) {
            faces.push(Face::D);
        }

        if self.contains(Self::B) {
            faces.push(Face::B);
        } else if self.contains(Self::F) {
            faces.push(Face::F);
        }

        if self.contains(Self::R) {
            faces.push(Face::R);
        } else if self.contains(Self::L) {
            faces.push(Face::L);
        }

        faces
    }
}