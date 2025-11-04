//! crates/core/src/lattice.rs
use core::vectorial::{Vector3, Vector2};
use core::nalgebra;
// --- Type alias for 3D Lattice Vectors ---
pub type LatticeBaseVector = Vector3;
pub type LatticeInPlaneVector = Vector2;

/// Represents the lattice vectors of the periodic structure.
#[derive(Debug, Clone, PartialEq)]
pub struct Lattice {
    /// First lattice basis vector (a1).
    pub a1: LatticeBaseVector,
    /// Second lattice basis vector (a2).
    pub a2: LatticeBaseVector,
    /// Third lattice basis vector (a3).
    pub a3: LatticeBaseVector,
}

impl Lattice {
    /// Returns the 2D in-plane lattice vectors (a1.x, a1.y) and (a2.x, a2.y).
    pub fn in_plane_vectors(&self) -> (LatticeInPlaneVector, LatticeInPlaneVector) {
        (self.a1.xy(), self.a2.xy())
    }

    /// Calculates the area of the 2D in-plane unit cell.
    pub fn unit_cell_area(&self) -> f64 {
        let (a1_2d, a2_2d) = self.in_plane_vectors();
        let m  = nalgebra::Matrix2::from_columns(&[a1_2d, a2_2d]);
        m.determinant().abs()*0.5
    }

    /// Calculates the unit cell volume.
    pub fn unit_cell_volume(&self) -> f64 {
        let v = self.a1.dot(&self.a2.cross(&self.a3));
        v.abs()
    }
}

/// Enum to define specific types of 2D lattices.
#[derive(Debug, Clone, PartialEq)]
pub enum LatticeType {
    Square(Lattice),
    Triangular(Lattice),
}

impl LatticeType {
    /// Creates a new square lattice with lattice constant `a`.
    pub fn new_square(a: f64, h: f64) -> Self {
        LatticeType::Square(Lattice {
            a1: Vector3::new(a, 0.0, 0.0),
            a2: Vector3::new(0.0, a, 0.0),
            a3: Vector3::new(0.0, 0.0, h),
        })
    }

    /// Creates a new triangular (hexagonal) lattice with lattice constant `a`.
    pub fn new_triangular(a: f64, h: f64) -> Self {
        LatticeType::Triangular(Lattice {
            a1: Vector3::new(a, 0.0, 0.0),
            a2: Vector3::new(a * 0.5, a * 0.86602540378, 0.0), // (a/2, a*sqrt(3)/2)
            a3: Vector3::new(0.0, 0.0, h),
        })
    }

    /// Provides a reference to the underlying Lattice struct.
    pub fn lattice(&self) -> &Lattice {
        match self {
            LatticeType::Square(lat) => lat,
            LatticeType::Triangular(lat) => lat,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lattice_type_constructors() {
        let a = 1e-6;
        let h = 0.25*a;
        let square_lat = LatticeType::new_square(a, h);
        let lattice = square_lat.lattice();

        assert_eq!(lattice.a1, Vector3::new(a, 0.0, 0.0));
        assert_eq!(lattice.a2, Vector3::new(0.0, a, 0.0));
        assert!((lattice.unit_cell_area() - a * a).abs() < 1e-12);
        assert!((lattice.unit_cell_volume() - a * a * h).abs() < 1e-12);

        let tri_lat = LatticeType::new_triangular(a, h);
        let lattice = tri_lat.lattice();
        let expected_area = a * (a * 0.86602540378);

        assert_eq!(lattice.a1, Vector3::new(a, 0.0, 0.0));
        assert_eq!(lattice.a2, Vector3::new(a * 0.5, a * 0.86602540378, 0.0));
        assert!((lattice.unit_cell_area() - expected_area).abs() < 1e-12);
        assert!((lattice.unit_cell_volume() - expected_area * h).abs() < 1e-12);
    }

    #[test]
    fn test_in_plane_vectors() {
        let a = 100.0;
        let h = 50.0;
        let square_lat = LatticeType::new_square(a, h);
        let lat = square_lat.lattice();
        let (a1_2d, a2_2d) = lat.in_plane_vectors();
        assert_eq!(a1_2d, Vector2::new(a, 0.0));
        assert_eq!(a2_2d, Vector2::new(0.0, a));
    }
}