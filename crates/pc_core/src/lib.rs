//! crates/pc_core/src/lib.rs
//!
//! Core data structures for photonic crystal analysis.

// Declare the modules. Rust will look for `material.rs`, `lattice.rs`, etc.
pub mod geometry;
pub mod lattice;
pub mod material;
pub mod photonic_crystal;
pub mod waveguide;

// Re-export the main public types for a clean API.
// Other crates can just `use pc_core::Material` as before.
pub use geometry::{AtomInCell, HoleShape, UnitCellBase};
pub use lattice::{Lattice, LatticeType, LatticeVector};
pub use material::Material;
pub use photonic_crystal::PhotonicCrystal;
pub use waveguide::{LayerType, Waveguide};