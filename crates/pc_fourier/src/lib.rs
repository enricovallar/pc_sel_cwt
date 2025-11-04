//! crates/pc_fourier/src/lib.rs
//!
//! Converts unit cell geometry into Fourier coefficients using a 2D FFT.

use num_complex::Complex;
use pc_core::{HoleShape, PhotonicCrystal, ShapeInCell, UnitCellBase};
use ndarray::Array2;
use ndarray_fft::{FftDirection, FftPlanner, FftShift};

/// Trait for calculating Fourier coefficients from a unit cell's base geometry.
pub trait UnitCellFourier {
    /// Generates the real-space $\epsilon(x, y)$ grid for the unit cell by
    /// rasterizing all shapes.
    ///
    /// * `pc` - Provides `epsilon_air`, `epsilon_background`, and `lattice_constant`.
    /// * `grid_size` - The resolution of the grid (e.g., 128 for 128x128).
    fn generate_grid(&self, pc: &PhotonicCrystal, grid_size: usize) -> Array2<f64>;

    /// Calculates all $\xi_{m,n}$ coefficients using a 2D FFT of the grid.
    ///
    /// Returns a 2D array of `Complex<f64>` where $\xi_{0,0}$ (the average
    /// dielectric constant) is at the center of the array.
    fn calculate_all_xi(
        &self,
        pc: &PhotonicCrystal,
        grid_size: usize,
    ) -> Array2<Complex<f64>> {
        
        // 1. Get the real-space epsilon grid by rasterizing the shapes
        let epsilon_grid = self.generate_grid(pc, grid_size);
        let n_sq = (grid_size * grid_size) as f64;

        // 2. Calculate the average epsilon ($\xi_{0,0}$)
        let eps_av = epsilon_grid.mean().unwrap();

        // 3. We are interested in the Fourier series of $(\epsilon(r) - \epsilon_{av})$
        let delta_epsilon_grid = epsilon_grid.mapv(|eps| eps - eps_av);

        // 4. Convert to complex for FFT
        let mut fft_grid = delta_epsilon_grid.mapv(|val| Complex::new(val, 0.0));

        // 5. Perform 2D FFT
        // The paper's definition uses a +i in the exponent.
        // A standard 'forward' FFT uses -i.
        // Therefore, we use an INVERSE FFT (which has +i) and normalize.
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_2d(grid_size, grid_size, FftDirection::Inverse);
        fft.process(&mut fft_grid);

        // 6. Normalize
        // The inverse FFT gives $\sum x_{jk} e^{+i...}$.
        // The Fourier coefficient $\xi_{m,n}$ is $\frac{1}{N^2} \sum ...$
        fft_grid.mapv_inplace(|c| c / n_sq);

        // 7. Shift the result
        // FFT output has (0,0) at index [0, 0]. We want (0,0) at the center.
        fft_grid.fftshift();

        // 8. Set the (0,0) component $\xi_{0,0}$
        // The (0,0) component of the *delta* grid is 0 by definition.
        // We replace it with the true average, $\epsilon_{av}$, as this is
        // the (0,0) component of the *original* $\epsilon(r)$ grid.
        let center = grid_size / 2;
        fft_grid[[center, center]] = Complex::new(eps_av, 0.0);

        fft_grid
    }
}

/// Implement the trait for the `UnitCellBase` struct from `pc_core`.
impl UnitCellFourier for UnitCellBase {
    fn generate_grid(&self, pc: &PhotonicCrystal, grid_size: usize) -> Array2<f64> {
        // Start with a grid filled with the background material
        let mut grid = Array2::from_elem((grid_size, grid_size), pc.epsilon_background);
        
        let a = pc.lattice_constant;
        // The pixel coordinate of the (0,0) center
        let center_pix_f = (grid_size as f64 - 1.0) / 2.0;

        // Loop over every shape in the base and "draw" it
        for ShapeInCell { shape, center } in &self.shapes {
            // Convert fractional center (e.g., 0.25) to real-space (e.g., 0.25 * a)
            let (cx_real, cy_real) = (center.0 * a, center.1 * a);

            match shape {
                HoleShape::Circle { radius } => {
                    let r_sq = radius * radius;
                    
                    for ((i, j), eps) in grid.indexed_iter_mut() {
                        // Convert pixel index (i, j) to real-space (x, y)
                        // This maps grid indices [0, grid_size-1] to [~-a/2, ~+a/2]
                        let x = (i as f64 - center_pix_f) * (a / grid_size as f64);
                        let y = (j as f64 - center_pix_f) * (a / grid_size as f64);

                        // Calculate distance to the shape's center
                        let dist_sq = (x - cx_real).powi(2) + (y - cy_real).powi(2);

                        if dist_sq <= r_sq {
                            *eps = pc.epsilon_air;
                        }
                    }
                }
                HoleShape::EquilateralTriangle { .. } => {
                    // Rasterizing rotated polygons is complex
                    // (e.g., requires a point-in-polygon test for each pixel)
                    todo!("Implement rasterization for EquilateralTriangle");
                }
                HoleShape::RightAngledIsosceles { .. } => {
                    todo!("Implement rasterization for RightAngledIsosceles");
                }
            }
        }
        grid
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use pc_core::{PhotonicCrystal, UnitCellBase};
    use std::f64::consts::PI;

    /// Helper to create a standard PC for testing
    fn get_test_pc(base: UnitCellBase) -> PhotonicCrystal {
        PhotonicCrystal {
            lattice_constant: 295e-9, // 295 nm
            base,
            epsilon_air: 1.0,
            epsilon_background: 12.7449,
        }
    }

    /// Helper to create a simple PC with a centered circular hole
    fn get_simple_pc(ff: f64) -> PhotonicCrystal {
        let a = 295e-9;
        let base = UnitCellBase::from_simple_circle(ff, a);
        get_test_pc(base)
    }

    #[test]
    fn test_rasterization_centered_circle() {
        let ff = 0.16;
        let grid_size = 128;
        let pc = get_simple_pc(ff);

        // Act
        let grid = pc.base.generate_grid(&pc, grid_size);

        // Assert
        let center = grid_size / 2;
        // Center pixel (0,0) should be air
        assert_eq!(grid[[center, center]], pc.epsilon_air);
        // Corner pixel (~a/2, ~a/2) should be background
        assert_eq!(grid[[0, 0]], pc.epsilon_background);

        // Check if the calculated filling factor from the grid is close
        let air_pixels = grid.iter().filter(|&&eps| (eps - pc.epsilon_air).abs() < 1e-9).count();
        let total_pixels = grid_size * grid_size;
        let calculated_ff = air_pixels as f64 / total_pixels as f64;
        
        // Allow ~1% error due to grid discretization
        assert!((calculated_ff - ff).abs() < 0.01, "Calculated FF {} was not close to {}", calculated_ff, ff);
    }

    #[test]
    fn test_rasterization_off_center_circle() {
        let grid_size = 128;
        let a = 295e-9;
        let radius = a * (0.16 / PI).sqrt();
        let mut base = UnitCellBase::new();
        // Add a circle centered at (0.25, 0.0) in fractional coordinates
        base.add_shape(HoleShape::Circle { radius }, (0.25, 0.0));
        let pc = get_test_pc(base);

        // Act
        let grid = pc.base.generate_grid(&pc, grid_size);
        
        // Assert
        let center = grid_size / 2;
        // The (0,0) center pixel should now be background
        assert_eq!(grid[[center, center]], pc.epsilon_background);
        
        // The pixel corresponding to (0.25, 0.0) should be air
        // (0.25 * 128) = 32. So index is center + 32
        let air_pixel_i = center + (0.25 * grid_size as f64).round() as usize;
        assert_eq!(grid[[center, air_pixel_i]], pc.epsilon_air);
    }

    #[test]
    fn test_fft_centered_circle() {
        let ff = 0.16;
        let grid_size = 64; // Smaller for faster test
        let pc = get_simple_pc(ff);

        // Act
        let xi_grid = pc.base.calculate_all_xi(&pc, grid_size);
        
        // Assert
        let center = grid_size / 2;

        // 1. Check $\xi_{0,0}$ (the average dielectric constant)
        let eps_av = ff * pc.epsilon_air + (1.0 - ff) * pc.epsilon_background;
        let xi_00 = xi_grid[[center, center]];
        
        // Use a tolerance. Grid mean is not *exactly* eps_av due to rasterization.
        assert!((xi_00.re - eps_av).abs() < 1e-3, "$\xi_{0,0}$ {} was not {}", xi_00.re, eps_av);
        assert!(xi_00.im.abs() < 1e-9);

        // 2. Check symmetry: $\xi_{m,n}$ should be purely real for a centered,
        // symmetric shape.
        let max_imag = xi_grid.iter().map(|c| c.im.abs()).fold(f64::MIN, f64::max);
        assert!(max_imag < 1e-9, "FFT coefficients should be real, max imag was {}", max_imag);

        // 3. Check symmetry: $\xi_{1,0}$ should equal $\xi_{0,1}$
        let xi_10 = xi_grid[[center, center + 1]]; // (m=1, n=0)
        let xi_01 = xi_grid[[center + 1, center]]; // (m=0, n=1)
        assert!((xi_10 - xi_01).norm() < 1e-9, "$\xi_{1,0}$ must equal $\xi_{0,1}$");
    }
    
    #[test]
    fn test_fft_off_center_circle() {
        let grid_size = 64;
        let a = 295e-9;
        let ff = 0.16;
        let radius = a * (ff / PI).sqrt();
        let mut base = UnitCellBase::new();
        base.add_shape(HoleShape::Circle { radius }, (0.25, 0.0)); // Shifted
        let pc = get_test_pc(base);

        // Act
        let xi_grid = pc.base.calculate_all_xi(&pc, grid_size);
        
        // Assert
        let center = grid_size / 2;

        // 1. Check $\xi_{0,0}$ (should still be the same average)
        let eps_av = ff * pc.epsilon_air + (1.0 - ff) * pc.epsilon_background;
        let xi_00 = xi_grid[[center, center]];
        assert!((xi_00.re - eps_av).abs() < 1e-3, "$\xi_{0,0}$ {} was not {}", xi_00.re, eps_av);

        // 2. Check broken symmetry: $\xi_{m,n}$ should now be COMPLEX
        // due to the phase shift from the translation.
        let xi_10 = xi_grid[[center, center + 1]]; // (m=1, n=0)
        assert!(xi_10.im.abs() > 1e-9, "$\xi_{1,0}$ should be complex");
        
        // 3. Check broken symmetry: $\xi_{1,0}$ should NOT equal $\xi_{0,1}$
        let xi_01 = xi_grid[[center + 1, center]]; // (m=0, n=1)
        assert!(xi_01.im.abs() < 1e-9, "$\xi_{0,1}$ should be real (no shift in y)");
        assert!((xi_10 - xi_01).norm() > 1e-9, "$\xi_{1,0}$ should not equal $\xi_{0,1}$");
    }
}