# geom_builder

Library for rasterizing 3D photonic crystal waveguide structures into orthogonal grids.

## Overview

This crate provides functionality to convert complex photonic crystal waveguide geometries (with potentially non-orthogonal lattices) into regular 3D grids suitable for numerical simulations. The main function performs lattice rectification to accurately represent the structure on a Cartesian mesh.

## Features

- **3D Rasterization**: Convert waveguide structures to volumetric epsilon grids
- **Lattice Rectification**: Accurately sample non-orthogonal photonic crystal lattices
- **Layer Stack Support**: Handle arbitrary vertical layer configurations
- **Flexible Resolution**: User-configurable grid resolution (x, y, z)
- **Efficient Sampling**: Optimized for performance with large grids

## Usage

### As a Library

```rust
use geom_builder::rasterize_waveguide_3d;
use core::{Material, LatticeType, UnitCellBase, PhotonicCrystal, Waveguide};

// Define materials
let air = Material::new_from_eps(1.0);
let gaas = Material::new_from_eps(12.7449);

// Create photonic crystal geometry
let a = 295e-9; // lattice constant
let lattice = LatticeType::new_square(a);
let base = UnitCellBase::from_simple_circle(0.16, lattice.lattice(), air);
let geom = PhotonicCrystal { lattice, base };

// Build waveguide
let wg = Waveguide::new_from_table_i(geom, gaas);

// Rasterize to 3D grid
let resolution = (64, 64, 128);
let grid = rasterize_waveguide_3d(&wg, resolution);
```

### Examples

Run the included examples:

```bash
# Full workflow with HDF5 output
cargo run --example build_waveguide -p geom_builder

# Simple rasterization with statistics
cargo run --example simple_rasterize -p geom_builder
```

## Testing

Run the test suite:

```bash
cargo test -p geom_builder
```

Tests include:
- Basic rasterization correctness
- Multiple resolution validation
- Grid symmetry checks

## Documentation

Generate and view the API documentation:

```bash
cargo doc -p geom_builder --open
```

## Dependencies

- `ndarray` - N-dimensional array operations
- `core` - Core photonic crystal data structures
- `io` - HDF5 input/output (examples only)
- `tracing` - Structured logging

## Performance Notes

- Resolution scales as O(res_x × res_y × res_z)
- Typical resolutions: 64×64×128 to 128×128×256
- Memory usage is approximately 8 bytes per voxel
- Consider starting with lower resolutions for testing

## License

See the workspace root for license information.
