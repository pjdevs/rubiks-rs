use core::fmt;

pub type FaceMask = u8;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[repr(u8)]
pub enum Face {
    U = 1 << 0,
    D = 1 << 1,
    F = 1 << 2,
    B = 1 << 3,
    L = 1 << 4,
    R = 1 << 5,
}

impl Face {
    pub const fn get_mask(&self) -> FaceMask {
        *self as FaceMask
    }
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
