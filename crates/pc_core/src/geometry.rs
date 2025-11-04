//! crates/pc_core/src/geometry.rs

use super::lattice::Lattice;
use super::material::Material;
use std::f64::consts::PI;

/// Represents the physical geometry of a single hole.
#[derive(Debug, Clone, PartialEq)]
pub enum HoleShape {
    Circle { radius: f64 },
    EquilateralTriangle {
        side: f64,
        rotation_degrees: f64,
    },
    RightAngledIsosceles {
        leg: f64,
        rotation_degrees: f64,
    },
}

/// Represents a single atom (a shape + material) placed within the unit cell.
#[derive(Debug, Clone, PartialEq)]
pub struct AtomInCell {
    pub shape: HoleShape,
    /// Center in fractional coordinates (s, t)
    pub center: (f64, f64),
    /// The material of this atom.
    pub material: Material,
}

/// Defines the complete "base" of the unit cell as a collection of atoms.
#[derive(Debug, Clone, Default)]
pub struct UnitCellBase {
    pub atoms: Vec<AtomInCell>,
}

impl UnitCellBase {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an atom (shape + material) to the base.
    pub fn add_atom(&mut self, shape: HoleShape, center: (f64, f64), material: Material) {
        self.atoms.push(AtomInCell {
            shape,
            center,
            material,
        });
    }

    /// Helper to create a single, centered circular hole from a filling factor (f).
    pub fn from_simple_circle(
        filling_factor: f64,
        lattice: &Lattice,
        hole_material: Material,
    ) -> Self {
        let area = lattice.unit_cell_area();
        // $f = \pi r^2 / Area \implies r = \sqrt{f * Area / \pi}$
        let radius = (filling_factor * area / PI).sqrt();
        let mut base = Self::new();
        base.add_atom(
            HoleShape::Circle { radius },
            (0.0, 0.0),
            hole_material,
        );
        base
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lattice::{LatticeType}; // Use crate-relative paths
    use crate::material::Material;

    #[test]
    fn test_simple_circle_with_new_lattice_and_material() {
        let ff = 0.16;
        let a = 295e-9;
        let air = Material::new_from_eps(1.0);
        
        let lat_type_sq = LatticeType::new_square(a);
        let lat_sq = lat_type_sq.lattice();
        let base_sq = UnitCellBase::from_simple_circle(ff, lat_sq, air);
        
        let expected_radius_sq = (ff * lat_sq.unit_cell_area() / PI).sqrt();
        
        assert_eq!(base_sq.atoms.len(), 1);
        assert_eq!(base_sq.atoms[0].material, air);
        assert_eq!(base_sq.atoms[0].center, (0.0, 0.0));
        match base_sq.atoms[0].shape {
            HoleShape::Circle { radius } => {
                assert!((radius - expected_radius_sq).abs() < 1e-12);
            }
            _ => panic!(),
        }
    }
}