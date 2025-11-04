//! crates/core/src/lib.rs
//!
//! Core data structures for photonic crystal analysis.


pub mod material;
pub mod vectorial;
pub mod shapes;

// re-export nlaalgebra for convenience
pub use nalgebra;
