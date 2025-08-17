# rubiks-rs 🧩

`rubiks-rs` is a **Rubik’s Cube 3×3** implementation in Rust.  
The project provides an efficient internal representation of the cube, supports manipulation and state inspection, and already includes a **blindfolded solver (Old Pochmann method)**.  

## Current Features 🚀
- Representation of a 3×3  
- Apply moves and sequences (standard notation)  
- Inspect cube state (pieces, orientations, permutations)  
- **Blindfolded Old Pochmann solver** (corners and edges)  

## Planned Features 🛠️
- Scramble generation via **random cube states**
- **3D visualizer** to observe manipulations and solves  
- Support for larger NxN cubes (4×4, 5×5, …)  
- Additional solving methods and optimized solvers and cube representation

## Example Usage
```rust
use rubiks::cube::{Cube, CubeMove::*};
use rubiks::solvers::pochmann::PochmannSolver;
use rubiks::stickers::CubeStickerLocation;

fn main() {
    // Solved cube
    let mut cube = Cube::solved();

    // Apply a scramble
    cube.apply_moves(&vec![R, U, Rp, Up]);

    // Old Pochmann solve
    let solver = PochmannSolver {
        buffer_corner: CubeStickerLocation::UBL,
        buffer_edge: CubeStickerLocation::UR,
    };
    let solution = solver.solve(&cube);

    println!("C: {:?}", solution.corner_cycles);
    println!("E: {:?}", solution.edge_cycles);
}
```

## Project Goals

- Provide a clean and idiomatic Rust base for Rubik’s Cube manipulation
- Enable experimentation with solving algorithms and visualization
- Serve as a learning resource for Rust applied to a concrete problem
