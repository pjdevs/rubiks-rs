use crate::cube_constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

    pub const fn corner_opposite(&self) -> Twist {
        Twist((STICKERS_ON_CORNERS - self.0) % STICKERS_ON_CORNERS)
    }

    pub const fn edge_opposite(&self) -> Twist {
        Twist((STICKERS_ON_EDGES - self.0) % STICKERS_ON_EDGES)
    }

    pub const fn number_of_twists(&self) -> u8 {
        self.0
    }
}