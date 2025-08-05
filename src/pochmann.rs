use crate::cube::{Cube, CubeSticker, CornerSticker, CornerSticker::*, EdgeSticker, EdgeSticker::*};

pub struct PochmannSolver {
    buffer_corner: CornerSticker,
    buffer_edge: EdgeSticker,
}

pub struct PochmannSolution {
    corner_cycles: Vec<Vec<CornerSticker>>,
    edge_cycles: Vec<Vec<EdgeSticker>>
}

impl PochmannSolver {
    pub fn solve(&self, cube: &Cube) -> PochmannSolution {
        let mut cycles = Vec::<Vec<CornerSticker>>::new();
        let mut current_cycle = Vec::<CornerSticker>::new();
        let mut cycle_start = self.buffer_corner;
        let mut next = cube.get_sticker_color(cycle_start.into());

        loop {
            while !next.is_same_cubie(&cycle_start.into()) {
                current_cycle.push(next.try_into().expect("Corner must be a corner"));
                next = cube.get_sticker_color(next)
            }

            cycle_start = self.pick_new_random_cycle_start(&cube);

            cycles.push(current_cycle.clone());
            current_cycle.clear();

            if cycle_start == self.buffer_corner {
                break;
            }
        }
         

        PochmannSolution {
            corner_cycles: vec![
                vec![UBL, BLU, UBR, BRU]
            ],
            edge_cycles: vec![
                vec![UB, BU, UF, FU]
            ]
        }
    }
    
    fn pick_new_random_cycle_start(&self, cube: &Cube) -> CornerSticker {
        todo!()
    }
}