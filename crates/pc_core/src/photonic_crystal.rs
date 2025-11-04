//! crates/pc_core/src/photonic_crystal.rs

use super::geometry::UnitCellBase;
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
    use crate::geometry::UnitCellBase; // Use crate-relative paths
    use crate::lattice::LatticeType;
    use crate::material::Material;

    #[test]
    fn test_photonic_crystal_struct_is_pure_geometry() {
        let a = 295e-9;
        let air = Material::new_from_eps(1.0);
        let lattice = LatticeType::new_square(a);
        let base = UnitCellBase::from_simple_circle(0.16, lattice.lattice(), air);
        
        let pc_geom = PhotonicCrystal {
            lattice,
            base,
        };
        
        // This struct just holds geometry
        assert_eq!(pc_geom.lattice.lattice().a1, (a, 0.0, 0.0));
        assert_eq!(pc_geom.base.atoms.len(), 1);
        assert_eq!(pc_geom.base.atoms[0].material, air);
    }
}