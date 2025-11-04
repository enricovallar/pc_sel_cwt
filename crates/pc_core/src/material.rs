//! crates/pc_core/src/material.rs

/// Represents the physical properties of a material.
/// For now, assumes a diagonal, isotropic dielectric tensor.
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Material {
    /// Diagonal elements of the dielectric tensor $(\epsilon_x, \epsilon_y, \epsilon_z)$.
    pub epsilon_matrix: (f64, f64, f64),
}

impl Material {
    /// Creates a new isotropic material from a refractive index `n`.
    pub fn new_from_n(n: f64) -> Self {
        let eps = n * n;
        Self {
            epsilon_matrix: (eps, eps, eps),
        }
    }

    /// Creates a new isotropic material from a dielectric constant `eps`.
    pub fn new_from_eps(eps: f64) -> Self {
        Self {
            epsilon_matrix: (eps, eps, eps),
        }
    }

    /// Returns the in-plane dielectric constant for TE polarization (assumes $\epsilon_x$).
    pub fn in_plane_eps(&self) -> f64 {
        self.epsilon_matrix.0 // $\epsilon_x$
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_material_constructors() {
        let n = 1.5;
        let eps = n * n; // 2.25
        let mat_n = Material::new_from_n(n);
        let mat_eps = Material::new_from_eps(eps);

        assert_eq!(mat_n.epsilon_matrix, (eps, eps, eps));
        assert_eq!(mat_eps.epsilon_matrix, (eps, eps, eps));
        assert!((mat_n.in_plane_eps() - eps).abs() < 1e-12);
    }
}