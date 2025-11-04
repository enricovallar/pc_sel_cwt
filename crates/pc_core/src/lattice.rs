//! crates/pc_core/src/lattice.rs

// --- Type alias for 3D Lattice Vectors ---
pub type LatticeVector = (f64, f64, f64);

/// Represents the lattice vectors of the periodic structure.
#[derive(Debug, Clone, PartialEq)]
pub struct Lattice {
    /// First lattice basis vector (a1).
    pub a1: LatticeVector,
    /// Second lattice basis vector (a2).
    pub a2: LatticeVector,
    /// Third lattice basis vector (a3).
    pub a3: LatticeVector,
}

impl Lattice {
    /// Returns the 2D in-plane lattice vectors (a1.x, a1.y) and (a2.x, a2.y).
    pub fn in_plane_vectors(&self) -> ((f64, f64), (f64, f64)) {
        ((self.a1.0, self.a1.1), (self.a2.0, self.a2.1))
    }

    /// Calculates the area of the 2D in-plane unit cell.
    pub fn unit_cell_area(&self) -> f64 {
        let (a1_2d, a2_2d) = self.in_plane_vectors();
        // Area = |a1.x * a2.y - a1.y * a2.x|
        (a1_2d.0 * a2_2d.1 - a1_2d.1 * a2_2d.0).abs()
    }

    /// Calculates the unit cell volume.
    pub fn unit_cell_volume(&self) -> f64 {
        // Volume = a1 . (a2 x a3)
        let cross = (
            self.a2.1 * self.a3.2 - self.a2.2 * self.a3.1,
            self.a2.2 * self.a3.0 - self.a2.0 * self.a3.2,
            self.a2.0 * self.a3.1 - self.a2.1 * self.a3.0,
        );
        (self.a1.0 * cross.0 + self.a1.1 * cross.1 + self.a1.2 * cross.2).abs()
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
    pub fn new_square(a: f64) -> Self {
        LatticeType::Square(Lattice {
            a1: (a, 0.0, 0.0),
            a2: (0.0, a, 0.0),
            a3: (0.0, 0.0, 0.0), // z-vector not used for 2D periodicity
        })
    }

    /// Creates a new triangular (hexagonal) lattice with lattice constant `a`.
    pub fn new_triangular(a: f64) -> Self {
        LatticeType::Triangular(Lattice {
            a1: (a, 0.0, 0.0),
            a2: (a * 0.5, a * 0.86602540378, 0.0), // (a/2, a*sqrt(3)/2)
            a3: (0.0, 0.0, 0.0),
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
        let a = 100e-9;
        let square_lat = LatticeType::new_square(a);
        let lattice = square_lat.lattice();

        assert_eq!(lattice.a1, (a, 0.0, 0.0));
        assert_eq!(lattice.a2, (0.0, a, 0.0));
        assert!((lattice.unit_cell_area() - a * a).abs() < 1e-12);

        let tri_lat = LatticeType::new_triangular(a);
        let lattice = tri_lat.lattice();
        let expected_area = a * (a * 0.86602540378);

        assert_eq!(lattice.a1, (a, 0.0, 0.0));
        assert_eq!(lattice.a2, (a * 0.5, a * 0.86602540378, 0.0));
        assert!((lattice.unit_cell_area() - expected_area).abs() < 1e-12);
    }

    #[test]
    fn test_in_plane_vectors() {
        let a = 100.0;
        let square_lat = LatticeType::new_square(a);
        let lat = square_lat.lattice();
        let (a1_2d, a2_2d) = lat.in_plane_vectors();
        assert_eq!(a1_2d, (a, 0.0));
        assert_eq!(a2_2d, (0.0, a));
    }
}