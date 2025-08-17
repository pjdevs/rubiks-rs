// TODO to generalize
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CubeMove {
    U,
    R,
    Up,
    Rp,
}

impl CubeMove {
    pub fn inverted(&self) -> Self {
        match self {
            CubeMove::U => Self::Up,
            CubeMove::R => Self::Rp,
            CubeMove::Up => Self::U,
            CubeMove::Rp => Self::R,
        }
    }
}