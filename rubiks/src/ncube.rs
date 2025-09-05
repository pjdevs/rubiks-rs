// TODO just keep this here for later if want generic impl

const CORNER_COUNT: usize = 8;

const fn edge_count(n: usize) -> usize {
    12 * (n - 2)
}

const fn center_count(n: usize) -> usize {
    if n > 3 {
        let n_minus_2 = n - 2;
        6 * n_minus_2 * n_minus_2
    } else {
        0
    }
}

#[derive(Clone, Copy)]
struct NPiece {}

struct NCube<const N: usize, const CORNERS: usize, const EDGES: usize, const CENTERS: usize> {
    corners: [NPiece; CORNERS],
    edges: [NPiece; EDGES],
    centers: [NPiece; CENTERS],
}

impl<const N: usize, const CORNERS: usize, const EDGES: usize, const CENTERS: usize>
    NCube<N, CORNERS, EDGES, CENTERS>
{
    pub const fn new() -> Self {
        NCube {
            corners: [NPiece {}; CORNERS],
            edges: [NPiece {}; EDGES],
            centers: [NPiece {}; CENTERS],
        }
    }
}

const CUBE3: NCube<3, CORNER_COUNT, { edge_count(3) }, { center_count(3) }> = NCube::new();
