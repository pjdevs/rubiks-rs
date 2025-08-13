use crate::rubiks::faces::FaceMask;
use crate::rubiks::twist::Twist;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cubie {
    faces: FaceMask,
    twist: Twist,
}

impl Cubie {
    pub const UFR: Cubie = Cubie::corner(FaceMask::UFR);
    pub const UFL: Cubie = Cubie::corner(FaceMask::UFL);
    pub const UBL: Cubie = Cubie::corner(FaceMask::UBL);
    pub const UBR: Cubie = Cubie::corner(FaceMask::UBR);
    pub const DFR: Cubie = Cubie::corner(FaceMask::DFR);
    pub const DFL: Cubie = Cubie::corner(FaceMask::DFL);
    pub const DBL: Cubie = Cubie::corner(FaceMask::DBL);
    pub const DBR: Cubie = Cubie::corner(FaceMask::DBR);
    pub const UR: Cubie = Cubie::edge(FaceMask::UR);
    pub const UF: Cubie = Cubie::edge(FaceMask::UF);
    pub const UL: Cubie = Cubie::edge(FaceMask::UL);
    pub const UB: Cubie = Cubie::edge(FaceMask::UB);
    pub const DR: Cubie = Cubie::edge(FaceMask::DR);
    pub const DF: Cubie = Cubie::edge(FaceMask::DF);
    pub const DL: Cubie = Cubie::edge(FaceMask::DL);
    pub const DB: Cubie = Cubie::edge(FaceMask::DB);
    pub const FR: Cubie = Cubie::edge(FaceMask::FR);
    pub const FL: Cubie = Cubie::edge(FaceMask::FL);
    pub const BL: Cubie = Cubie::edge(FaceMask::BL);
    pub const BR: Cubie = Cubie::edge(FaceMask::BR);

    pub const fn is_corner(&self) -> bool {
        self.faces.is_corner()
    }

    pub const fn is_edge(&self) -> bool {
        self.faces.is_edge()
    }

    pub const fn twisted(&self, twist: Twist) -> Cubie {
        Cubie {
            faces: self.faces,
            twist: if self.is_corner() {
                self.twist.corner_add(&twist)
            } else {
                self.twist.edge_add(&twist)
            }
        }
    }

    const fn corner(faces: FaceMask) -> Cubie {
        assert!(faces.is_corner());

        Cubie {
            faces,
            twist: Twist::SOLVED
        }
    }

    const fn edge(faces: FaceMask) -> Cubie {
        assert!(faces.is_edge());

        Cubie {
            faces,
            twist: Twist::SOLVED
        }
    }
}