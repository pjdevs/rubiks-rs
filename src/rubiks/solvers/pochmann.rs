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
            println!("Cycle {:?}", start_location);

            // Build the cycle starting from `start`
            let mut current_cycle = Vec::new();
            for (next_location, does_solve) in PochmannSolver::iter_single_cycle(cube, &start_location, &self.buffer_corner) {
                println!("    {:?}", next_location);
                current_cycle.push(next_location);

                if does_solve {
                    solved_locations.insert(next_location.piece_location);
                }
            }

            cycles.push(current_cycle);

            // Find the next unsolved corner
            next_start = PochmannSolver::find_next_unsolved_sticker(cube, &solved_locations);
        }
        cycles
    }

    fn iter_single_cycle(cube: &Cube, start: &CubeStickerLocation, buffer: &CubeStickerLocation) -> impl Iterator<Item = (CubeStickerLocation, bool)> {
        std::iter::successors(
            Some(
                if start.piece_location == buffer.piece_location {
                    (cube.get_sticker_origin(&start), true)
                } else {
                    (*start, false)
                }
            ),
            |(prev, has_solved_last_piece)| {
                if prev.piece_location == start.piece_location && *has_solved_last_piece {
                    return None;
                }

                let next = cube.get_sticker_origin(prev);
                println!("    twist is {:?}", next.twist);
                if next.piece_location == buffer.piece_location {
                    None
                } else {
                    Some((next, true))
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
}