use std::collections::HashSet;

use crate::rubiks::cube::Cube;
use crate::rubiks::location::CubePieceLocation;
use crate::rubiks::stickers::CubeStickerLocation;
use crate::rubiks::twist::Twist;

pub struct PochmannSolver {
    pub buffer_corner: CubeStickerLocation,
    pub buffer_edge: CubeStickerLocation,
}

pub struct PochmannSolution {
    pub corner_cycles: Vec<Vec<CubeStickerLocation>>,
    pub edge_cycles: Vec<Vec<CubeStickerLocation>>
}

impl PochmannSolver {
    pub fn solve(&self, cube: &Cube) -> PochmannSolution {
        let corner_cycles = self.solve_corners_v2(cube);

        PochmannSolution {
            corner_cycles,
            edge_cycles: vec![]
        }
    }

    fn solve_corners_v2(&self, cube: &Cube) -> Vec<Vec<CubeStickerLocation>> {
        let mut solved_locations = HashSet::from([
            self.buffer_corner.piece_location
        ]);
        let mut cycles = Vec::new();

        // Start with the buffer, then continue with unsolved corners
        let mut next_start = Some(self.buffer_corner);

        while let Some(start_location) = next_start {
            // Build the cycle starting from `start`
            let mut current_cycle = Vec::new();
            for next_location in PochmannSolver::iter_single_cycle(cube, &start_location) {
                current_cycle.push(next_location);
                solved_locations.insert(next_location.piece_location);
            }

            // Include start if cycle was length 1 (flip)
            if current_cycle.len() == 1 && start_location != self.buffer_corner {
                current_cycle.push(start_location);
                solved_locations.insert(start_location.piece_location);
            }

            cycles.push(current_cycle);

            // Find the next unsolved corner
            next_start = PochmannSolver::find_next_unsolved_sticker(cube, &solved_locations);
        }
        cycles
    }

    fn iter_single_cycle(cube: &Cube, start_location: &CubeStickerLocation) -> impl Iterator<Item = CubeStickerLocation> {
        std::iter::successors(
            Some(cube.get_sticker_origin(&start_location)),
            |prev| {
                let next = cube.get_sticker_origin(prev);
                if next.piece_location == start_location.piece_location {
                    None
                } else {
                    Some(next)
                }
            }
        )
    }

    fn find_next_unsolved_sticker(cube: &Cube, solved_locations: &HashSet<CubePieceLocation>) -> Option<CubeStickerLocation> {
        cube
            .iter_corners()
            .find(|(location, piece)| !solved_locations.contains(location) && piece.get_original_location() != **location)
            .map(|(location, _)| CubeStickerLocation { piece_location: *location, twist: Twist::SOLVED })
    }

    // fn solve_corners(&self, cube: &Cube) -> Vec<Vec<CubeStickerLocation>> {
    //     // Results
    //     let mut cycles = Vec::<Vec<CubeStickerLocation>>::new();
    //     let mut current_cycle = Vec::<CubeStickerLocation>::new();

    //     // Loop state
    //     let mut solved_locations = HashSet::<CubePieceLocation>::new();
    //     let mut cycle_start_location: CubeStickerLocation;
    //     let mut next_location: CubeStickerLocation;
    //     let first = true;

    //     // All cycles loop
    //     loop {
    //         // Find start of cycle and init next one
    //         cycle_start_location = if !first {
    //             match self.find_non_solved_corner_location(&cube, &solved_locations) {
    //                 Some(location) => location,
    //                 None => break
    //             }
    //         } else {
    //             self.buffer_corner
    //         };
    //         next_location = cube.get_sticker_origin(&cycle_start_location);

    //         // Put the new cycle start into buffer (i.e. this is not the first cycle)
    //         if cycle_start_location != self.buffer_corner {
    //             current_cycle.push(cycle_start_location);
    //             solved_locations.insert(cycle_start_location.piece_location);
    //         }
            
    //         // Inside cycle loop, find next one until we get back to start
    //         loop {
    //             let next_corner = next_location.try_into().expect("Corner must be a corner");
                
    //             // Add solved cubie
    //             current_cycle.push(next_corner);
    //             solved_locations.insert(next_corner.piece_location);
    //             next_location = cube.get_sticker_origin(next_location);

    //             // Stop if we got back to start cubie
    //             if next_location.piece_location == cycle_start_location.piece_location {
    //                 break;
    //             }
    //         }

    //         // Save this cycle, clear, continue
    //         cycles.push(current_cycle.clone());
    //         current_cycle.clear();
    //     }

    //     cycles
    // }
    
    // fn find_non_solved_corner_location(&self, cube: &Cube, solved_locations: &HashSet<CubePieceLocation>) -> Option<CubeStickerLocation> {
    //     for (location, piece) in cube.iter_corners() {
    //         // If this cubie was already solved, skip it
    //         if solved_locations.contains(location) {
    //             continue;
    //         }

    //         // If cubie is solved, skip
    //         if piece.is_solved(location) {
    //             continue;
    //         }

    //         // Otrherwise found
    //         return Some(CubeStickerLocation { piece_location: *location, twist: Twist::SOLVED });
    //     }

    //     // All cubies are solved i.e. cube is solved :)
    //     None
    // }
}