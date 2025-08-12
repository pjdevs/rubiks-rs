use std::collections::HashSet;

use crate::cube::{CornerSticker, Cube, CubeElement, CubeSticker, EdgeSticker, Faces};

pub struct PochmannSolver {
    pub buffer_corner: CornerSticker,
    pub buffer_edge: EdgeSticker,
}

pub struct PochmannSolution {
    pub corner_cycles: Vec<Vec<CornerSticker>>,
    pub edge_cycles: Vec<Vec<EdgeSticker>>
}

impl PochmannSolver {
    pub fn solve(&self, cube: &Cube) -> PochmannSolution {
        let corner_cycles = self.solve_corners(cube);

        PochmannSolution {
            corner_cycles,
            edge_cycles: vec![]
        }
    }

    fn solve_corners(&self, cube: &Cube) -> Vec<Vec<CornerSticker>> {
        // Results
        let mut cycles = Vec::<Vec<CornerSticker>>::new();
        let mut current_cycle = Vec::<CornerSticker>::new();
        
        // Loop state
        let mut solved_corners = HashSet::<Faces>::new();
        let mut cycle_start: CornerSticker;
        let mut next: CubeSticker;

        // All cycles loop
        loop {
            // Find start of cycle and init next one
            match self.find_non_solved_corner(&cube, &solved_corners) {
                Some(location) => cycle_start = location,
                None => break
            }
            next = cube.get_sticker_at(cycle_start.into());

            // Put the new cycle start into buffer (i.e. this is not the first cycle)
            if cycle_start != self.buffer_corner {
                current_cycle.push(cycle_start);
                solved_corners.insert(cycle_start.get_face_mask());
            }
            
            // Inside cycle loop, find next one until we get back to start
            loop {
                let next_corner = next.try_into().expect("Corner must be a corner");
                
                // Add solved cubie
                current_cycle.push(next_corner);
                solved_corners.insert(next_corner.get_face_mask());
                next = cube.get_sticker_at(next);

                // Stop if we got back to start cubie
                if next.is_same_cubie(&cycle_start.into()) {
                    break;
                }
            }

            // Save this cycle, clear, continue
            cycles.push(current_cycle.clone());
            current_cycle.clear();
        }

        cycles
    }
    
    fn find_non_solved_corner(&self, cube: &Cube, solved_corners: &HashSet<Faces>) -> Option<CornerSticker> {
        for sticker in enum_iterator::all::<CornerSticker>() {
            // If this cubie was already solved, skip it
            if solved_corners.contains(&sticker.get_face_mask()) {
                continue;
            }

            // If cubie is solved, skip
            let actual = cube.get_sticker_at(CubeSticker::CornerSticker(sticker));
            if actual == CubeSticker::CornerSticker(sticker) {
                continue;
            }

            // Otrherwise found
            return Some(sticker);
        }

        // All cubies are solved i.e. cube is solved :)
        None
    }
}