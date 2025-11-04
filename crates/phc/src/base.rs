//! crates/core/src/geometry.rs
//! Defines the geometry of holes within the unit cell.
use core::material::{Material, CommonMaterials};
use core::shapes::HoleShape;





/// Represents a single atom (a shape + material) placed within the unit cell.
#[derive(Debug, Clone, PartialEq)]
pub struct AtomInCell {
    /// The shape of the hole.
    pub shape: HoleShape,
    /// Center in fractional coordinates (s, t, z) expressed in units of the lattice vectors.
    pub center: (f64, f64, f64),
    /// The material of this atom.
    pub material: Material,
}

impl Default for AtomInCell {
    fn default() -> Self {
        Self {
            shape: HoleShape::Circle { radius: 0.1 },
            center: (0.5, 0.5, 0.0),
            material: CommonMaterials::Air.into(),
        }
    }
}



/// Defines the complete "base" of the unit cell as a collection of atoms.
#[derive(Debug, Clone, Default)]
pub struct UnitCellBase {
    pub atoms: Vec<AtomInCell>,
    pub background_material: Material,
}

impl UnitCellBase {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an atom (shape + material) to the base.
    pub fn add_atom(&mut self, shape: HoleShape, center: (f64, f64, f64), material: Material) {
        self.atoms.push(AtomInCell {
            shape,
            center,
            material,
        });
    }    

    /// Creates a simple unit cell base with a single circular hole at the center.
    pub fn from_simple_circle(radius: f64, material: Material) -> Self {
        let mut base = Self::new();
        base.add_atom(
            HoleShape::Circle { radius },
            (0.5, 0.5, 0.0),
            material,
        );
        base
    }
}
#[cfg(test)]
mod tests {
    use core::shapes::HoleShape;
    use super::*;
    use core::material::Material;

    #[test]
    fn test_simple_circle_with_new_lattice_and_material() {
        let material = Material::new_from_eps(1.0);
        let mut base = UnitCellBase::new();
        base.add_atom(
            HoleShape::Circle { radius: 0.2 },
            (0.5, 0.5, 0.0),
            material,
        );
        assert_eq!(base.atoms.len(), 1);
        assert_eq!(base.atoms[0].shape, HoleShape::Circle { radius: 0.2 });
        assert_eq!(base.atoms[0].center, (0.5, 0.5, 0.0));
        assert_eq!(base.atoms[0].material, material);
        
    }
}