use std::collections::HashSet;

use crate::cube::Cube;
use crate::location::CubePieceLocation;
use crate::piece::CubePiece;
use crate::stickers::CubeStickerLocation;
use crate::twist::Twist;

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
        let corner_cycles = Self::solve_pieces(cube, &self.buffer_corner);
        let edge_cycles = Self::solve_pieces(cube, &self.buffer_edge);

        PochmannSolution {
            corner_cycles,
            edge_cycles
        }
    }

    fn solve_pieces(cube: &Cube, buffer: &CubeStickerLocation) -> Vec<Vec<CubeStickerLocation>> {
        let mut solved_locations = HashSet::from([
            buffer.piece_location
        ]);
        let mut cycles = Vec::new();

        // Start with the buffer, then continue with unsolved corners
        let mut next_start = Some(*buffer);

        while let Some(start_location) = next_start {
            println!("Cycle {:?}", start_location);

            // Build the cycle starting from `start`
            let mut current_cycle = Vec::new();
            for (next_location, does_solve) in PochmannSolver::iter_single_cycle(cube, &start_location, &buffer) {
                println!("    {:?}", next_location);
                current_cycle.push(next_location);

                if does_solve {
                    solved_locations.insert(next_location.piece_location);
                }
            }

            cycles.push(current_cycle);

            // Find the next unsolved corner
            next_start = PochmannSolver::find_next_unsolved_sticker(cube, &solved_locations, buffer);
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

    fn find_next_unsolved_sticker(cube: &Cube, solved_locations: &HashSet<CubePieceLocation>, buffer: &CubeStickerLocation) -> Option<CubeStickerLocation> {
        if buffer.piece_location.is_corner() {
            Self::filter_unsolved_pieces(cube.iter_corners(), solved_locations)
        } else {
            Self::filter_unsolved_pieces(cube.iter_edges(), solved_locations)
        }
    }

    fn filter_unsolved_pieces<'a>(mut it: impl Iterator<Item = (&'a CubePieceLocation, &'a CubePiece)>, solved_locations: &HashSet<CubePieceLocation>,) -> Option<CubeStickerLocation> {
        it
            .find(|(location, piece)| !solved_locations.contains(location) && piece.get_original_location() != **location)
            .map(|(location, _)| CubeStickerLocation { piece_location: *location, twist: Twist::SOLVED })
    }
}