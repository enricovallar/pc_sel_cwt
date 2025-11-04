//! crates/core/src/material.rs
use super::vectorial::{Matrix3, Vector3};
/// Alias for the dielectric tensor represented as a 3x3 matrix.
pub type DielectricTensor = Matrix3;

/// Represents either isotropic or anisotropic dielectric constants.
pub enum DielectricConstant {
    Isotropic(f64),
    Anisotropic(DielectricTensor),
}

/// Represents either isotropic or anisotropic refractive indices.
pub enum RefractiveIndex {
    Isotropic(f64),
    Anisotropic(Vector3),
}

/// Dielectric Constant of Silicon at telecom wavelengths (~1.55 um)
pub const EPSILON_SILICON: f64 = 11.68;
/// Dielectric Constant of Silicon Dioxide at telecom wavelengths (~1.55 um)
pub const EPSILON_SILICON_DIOXIDE: f64 = 2.1;
/// Dielectric Constant of Indium Phosphide at telecom wavelengths (~1.55 um)
pub const EPSILON_INDIUM_PHOSPHIDE: f64 = 3.17;
/// Dielectric Constant of Vacuum
pub const EPSILON_VACUUM: f64 = 1.0;
/// Dielectric Constant of Air
pub const EPSILON_AIR: f64 = 1.0;


/// Represents the physical properties of a material.
/// For now, assumes a diagonal, isotropic dielectric tensor.
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Material {
    /// Diagonal elements of the dielectric tensor $(\epsilon_x, \epsilon_y, \epsilon_z)$.
    pub epsilon_matrix: DielectricTensor,
}


impl Material {
    /// Creates a new isotropic material from a refractive index `n`.
    pub fn new_from_n(n: f64) -> Self {
        let eps = n * n;
        Self {
            epsilon_matrix: DielectricTensor::from_diagonal_element(eps),
        }
    }

    /// Creates a new isotropic material from a dielectric constant `eps`.
    pub fn new_from_eps(eps: f64) -> Self {
        Self {
            epsilon_matrix: DielectricTensor::from_diagonal_element(eps),
        }
    }
    
    /// Creates a new anisotropic material from given dielectric constants, 
    /// assuming by default a diagonal tensor.
    pub fn new_anisotropic(eps_x: f64, eps_y: f64, eps_z: f64) -> Self {
        Self {
            epsilon_matrix: DielectricTensor::from_diagonal(&Vector3::new(eps_x, eps_y, eps_z)),
        }
    }

    /// Creates a new anisotropic material from a full dielectric tensor.
    pub fn new_from_tensor(epsilon_matrix: DielectricTensor) -> Self {
        Self { epsilon_matrix }
    }

    /// Check if the material is isotropic.
    pub fn is_isotropic(&self) -> bool {
        let d = self.epsilon_matrix.diagonal();
        (d[0] == d[1]) && (d[1] == d[2])
    }   

    /// Check if the material is anisotropic.
    pub fn is_anisotropic(&self) -> bool {
        !self.is_isotropic()
    }

    /// Get the refractive index.
    pub fn refractive_index(&self) -> RefractiveIndex {
        let d = self.epsilon_matrix.diagonal();
        if self.is_isotropic() {
            RefractiveIndex::Isotropic(d[0].sqrt())
        } else {
            RefractiveIndex::Anisotropic(Vector3::new(d[0].sqrt(), d[1].sqrt(), d[2].sqrt()))
        }
    }

    /// Get the dielectric constant
    pub fn dielectric_constants(&self) -> DielectricConstant {
        if self.is_isotropic() {
            let eps = self.epsilon_matrix[(0, 0)];
            DielectricConstant::Isotropic(eps)
        } else {
            DielectricConstant::Anisotropic(self.epsilon_matrix)
        }
    }
}

impl Default for Material {
    /// Default material is isotropic silicon.
    fn default() -> Self {
        Self::new_from_eps(EPSILON_SILICON)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_creation() {
        let mat_iso = Material::new_from_n(3.0);
        assert_eq!(
            mat_iso.epsilon_matrix,
            DielectricTensor::from_diagonal_element(9.0)
        );

        let mat_eps = Material::new_from_eps(4.0);
        assert_eq!(
            mat_eps.epsilon_matrix,
            DielectricTensor::from_diagonal_element(4.0)
        );

        let mat_aniso = Material::new_anisotropic(2.0, 3.0, 4.0);
        assert_eq!(
            mat_aniso.epsilon_matrix,
            DielectricTensor::from_diagonal(&Vector3::new(2.0, 3.0, 4.0)),
        );

        let tensor = DielectricTensor::new(
            1.0, 0.1, 0.2,
            0.1, 2.0, 0.3,
            0.2, 0.3, 3.0,
        );
        let mat_tensor = Material::new_from_tensor(tensor);
        assert_eq!(mat_tensor.epsilon_matrix, tensor);          
    }
}

pub enum CommonMaterials {
    Vacuum,
    Air,
    Silicon,
    SiliconDioxide,
    IndiumPhosphide,
}

impl From<CommonMaterials> for Material {
    fn from(mat: CommonMaterials) -> Self {
        match mat {
            CommonMaterials::Vacuum => Material::new_from_eps(EPSILON_VACUUM),
            CommonMaterials::Air => Material::new_from_eps(EPSILON_AIR),
            CommonMaterials::Silicon => Material::new_from_eps(EPSILON_SILICON),
            CommonMaterials::IndiumPhosphide => Material::new_from_eps(EPSILON_INDIUM_PHOSPHIDE),
            CommonMaterials::SiliconDioxide => Material::new_from_eps(EPSILON_SILICON_DIOXIDE),
        }
    }
}