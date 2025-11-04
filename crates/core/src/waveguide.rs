// //! crates/core/src/waveguide.rs

// use super::material::Material;
// use super::photonic_crystal::PhotonicCrystal;

// /// Represents a single layer in the waveguide stack.
// #[derive(Debug, Clone)]
// pub enum LayerType {
//     Simple {
//         name: String,
//         thickness: f64,
//         material: Material,
//     },
//     PhotonicCrystal {
//         name: String,
//         thickness: f64,
//         /// The geometric definition of the PC.
//         definition: PhotonicCrystal,
//         /// The material of the "background" slab.
//         background_material: Material,
//     },
// }

// impl LayerType {
//     /// Helper method to get the thickness of any layer type.
//     pub fn thickness(&self) -> f64 {
//         match self {
//             LayerType::Simple { thickness, .. } => *thickness,
//             LayerType::PhotonicCrystal { thickness, .. } => *thickness,
//         }
//     }

//     /// Helper method to get the name of any layer type.
//     pub fn name(&self) -> &str {
//         match self {
//             LayerType::Simple { name, .. } => name,
//             LayerType::PhotonicCrystal { name, .. } => name,
//         }
//     }
// }

// /// Represents the complete multilayer waveguide structure.
// #[derive(Debug, Clone)]
// pub struct Waveguide {
//     pub layers: Vec<LayerType>,
// }

// impl Waveguide {
//     /// Creates a new waveguide based on Table I.
//     pub fn new_from_table_i(
//         definition: PhotonicCrystal,
//         background_material: Material,
//     ) -> Self {
//         Self {
//             layers: vec![
//                 LayerType::Simple {
//                     name: "n-clad (AlGaAs)".into(),
//                     thickness: 1.5e-6,
//                     material: Material::new_from_eps(11.0224),
//                 },
//                 LayerType::Simple {
//                     name: "Active".into(),
//                     thickness: 0.0885e-6,
//                     material: Material::new_from_eps(12.8603),
//                 },
//                 LayerType::PhotonicCrystal {
//                     name: "PC".into(),
//                     thickness: 0.1180e-6,
//                     definition,
//                     background_material: background_material,
//                 },
//                 LayerType::Simple {
//                     name: "GaAs".into(),
//                     thickness: 0.0590e-6,
//                     material: Material::new_from_eps(12.7449),
//                 },
//                 LayerType::Simple {
//                     name: "p-clad (AlGaAs)".into(),
//                     thickness: 1.5e-6,
//                     material: Material::new_from_eps(11.0224),
//                 },
//             ],
//         }
//     }

//     /// Finds the index of the first PhotonicCrystal layer.
//     pub fn layer_index(&self) -> Option<usize> {
//         self.layers
//             .iter()
//             .position(|layer| matches!(layer, LayerType::PhotonicCrystal { .. }))
//     }

//     /// Gets a reference to the first PhotonicCrystal *layer*.
//     pub fn get_layer(&self) -> Option<&LayerType> {
//         self.layers.iter().find(|layer| {
//             matches!(layer, LayerType::PhotonicCrystal { .. })
//         })
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::geometry::UnitCellBase; // Crate-relative paths
//     use crate::lattice::LatticeType;
//     use crate::material::Material;
//     use crate::photonic_crystal::PhotonicCrystal;

//     // Helper to create a default PC for testing
//     fn create_test_pc() -> (PhotonicCrystal, Material) {
//         let a = 295e-9;
//         let air = Material::new_from_eps(1.0);
//         let gaas = Material::new_from_eps(12.7449);
        
//         let lattice = LatticeType::new_square(a);
//         let base = UnitCellBase::from_simple_circle(0.16, lattice.lattice(), air);
        
//         let geom = PhotonicCrystal {
//             lattice,
//             base,
//         };
//         (geom, gaas)
//     }

//     #[test]
//     fn test_waveguide_creation_with_material_pc() {
//         let (geom, gaas) = create_test_pc();
//         let wg = Waveguide::new_from_table_i(geom.clone(), gaas);

//         assert_eq!(wg.layers.len(), 5);
//         assert_eq!(wg.layers[2].name(), "PC");

//         // Check that the PC definition was correctly inserted
//         match &wg.layers[2] {
//             LayerType::PhotonicCrystal { definition, background_material, .. } => {
//                 assert_eq!(
//                     definition.lattice.lattice().a1,
//                     geom.lattice.lattice().a1
//                 );
//                 assert_eq!(definition.base.atoms.len(), 1);
//                 assert_eq!(*background_material, gaas);
//             }
//             _ => panic!("Layer 2 was not a PhotonicCrystal type"),
//         }
        
//         // Check simple layer
//          match &wg.layers[0] {
//             LayerType::Simple { material, .. } => {
//                 assert_eq!(material.in_plane_eps(), 11.0224);
//             }
//             _ => panic!("Layer 0 was not a Simple type"),
//         }
//     }

//     #[test]
//     fn test_layer_index_and_getter() {
//         let (geom, gaas) = create_test_pc();
//         let wg = Waveguide::new_from_table_i(geom, gaas);

//         assert_eq!(wg.layer_index(), Some(2));
        
//         let retrieved_layer = wg.get_layer().unwrap();
//         match retrieved_layer {
//             LayerType::PhotonicCrystal { definition, .. } => {
//                  assert_eq!(definition.lattice.lattice().a1, (295e-9, 0.0, 0.0));
//             }
//             _ => panic!()
//         }
//     }
// }