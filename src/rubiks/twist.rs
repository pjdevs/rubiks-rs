use crate::rubiks::cube_constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Twist(u8);

impl Twist {
    pub const SOLVED: Twist = Twist(0);
    pub const CW_120: Twist = Twist(1);
    pub const CW_240: Twist = Twist(2);
    pub const FLIPPED: Twist = Twist(1);

    pub const fn corner_add(&self, other: &Twist) -> Twist {
        Twist((self.0 + other.0) % STICKERS_ON_CORNERS)
    }

    pub const fn edge_add(&self, other: &Twist) -> Twist {
        Twist((self.0 + other.0) % STICKERS_ON_EDGES)
    }
}