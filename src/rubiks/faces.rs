use core::fmt;
use bitmask_enum::bitmask;
use crate::rubiks::cube_constants::*;

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
    UB = Self::U.or(Self::B).bits,
    UR = Self::U.or(Self::R).bits,
    UF = Self::U.or(Self::F).bits,
    UL = Self::U.or(Self::L).bits,
    DB = Self::D.or(Self::B).bits,
    DR = Self::D.or(Self::R).bits,
    DF = Self::D.or(Self::L).bits,
    DL = Self::D.or(Self::F).bits,
    BR = Self::B.or(Self::R).bits,
    BL = Self::B.or(Self::L).bits,
    FR = Self::F.or(Self::R).bits,
    FL = Self::F.or(Self::L).bits,
    UFR = Self::U.or(Self::F).or(Self::R).bits,
    UFL = Self::U.or(Self::F).or(Self::L).bits,
    UBL = Self::U.or(Self::B).or(Self::L).bits,
    UBR = Self::U.or(Self::B).or(Self::R).bits,
    DFR = Self::D.or(Self::F).or(Self::R).bits,
    DFL = Self::D.or(Self::F).or(Self::L).bits,
    DBL = Self::D.or(Self::B).or(Self::L).bits,
    DBR = Self::D.or(Self::B).or(Self::R).bits,
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