//! crates/core/src/photonic_crystal.rs

use super::base::UnitCellBase;
use super::lattice::LatticeType;

/// Defines the 2D periodic geometry (lattice + base).
/// This struct no longer contains material properties directly.
#[derive(Debug, Clone)]
pub struct PhotonicCrystal {
    pub lattice: LatticeType,
    pub base: UnitCellBase,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::UnitCellBase;
    use crate::lattice::LatticeType;
    use core::material::Material;

    #[test]
    fn test_photonic_crystal_struct_is_pure_geometry() {
        let a = 295e-9;
        let h: f64 = 0.25*a;
        let air = Material::new_from_eps(1.0);
        let lattice = LatticeType::new_square(a, h);
        let base = UnitCellBase::from_simple_circle(0.16, air);
        
        let geom = PhotonicCrystal {
            lattice,
            base,
        };
        
        // This struct just holds geometry
        assert_eq!(geom.lattice.lattice().a1, core::vectorial::Vector3::new(a, 0.0, 0.0));
        assert_eq!(geom.base.atoms.len(), 1);
        assert_eq!(geom.base.atoms[0].material, air);
    }
}


impl PhotonicCrystal {
    /// Creates a new photonic crystal structure given lattice and base.
    pub fn new(lattice: LatticeType, base: UnitCellBase) -> Self {
        Self { lattice, base }
    }    
}