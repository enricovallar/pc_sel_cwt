/// Type alias for 3D Vectors
pub type Vector3 = nalgebra::Vector3<f64>;


// Test module for Vector3D
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vector_operations() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let v3 = v1 + v2;
        assert_eq!(v3, Vector3::new(5.0, 7.0, 9.0));
        let v4 = v2 - v1;
        assert_eq!(v4, Vector3::new(3.0, 3.0, 3.0));
        let v5 = v1 * 2.0;
        assert_eq!(v5, Vector3::new(2.0, 4.0, 6.0));
        let v6 = v2 / 2.0;
        assert_eq!(v6, Vector3::new(2.0, 2.5, 3.0));
    }
    #[test]
    fn test_dot_product() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0);
    }
    #[test]
    fn test_magnitude() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        let mag = v.magnitude();
        assert_eq!(mag, 5.0);
    }
    #[test]
    fn test_normalize() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        let norm = v.normalize();
        assert!((norm.magnitude() - 1.0).abs() < 1e-12);
    }
    #[test]
    fn test_distance() {
        let p1 = Vector3::new(1.0, 2.0, 3.0);
        let p2 = Vector3::new(4.0, 6.0, 3.0);
        let dist = p1.metric_distance(&p2);
        assert_eq!(dist, 5.0);
    }
    #[test]
    fn test_cross_product() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);
        assert_eq!(cross, Vector3::new(0.0, 0.0, 1.0));
    }
}

/// Type alias for 2D Vectors
pub type Vector2 = nalgebra::Vector2<f64>;

// Test module for Vector2D
#[cfg(test)]
mod vector2_tests {

    use super::*;
    #[test]
    fn test_vector2_operations() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(3.0, 4.0);
        let v3 = v1 + v2;
        assert_eq!(v3, Vector2::new(4.0, 6.0));
        let v4 = v2 - v1;
        assert_eq!(v4, Vector2::new(2.0, 2.0));
        let v5 = v1 * 2.0;
        assert_eq!(v5, Vector2::new(2.0, 4.0));
        let v6 = v2 / 2.0;
        assert_eq!(v6, Vector2::new(1.5, 2.0));
    }
    #[test]
    fn test_vector2_dot_product() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(3.0, 4.0);
        let dot = v1.dot(&v2);
        assert_eq!(dot, 11.0);
    }
    #[test]
    fn test_vector2_magnitude() {
        let v = Vector2::new(3.0, 4.0);
        let mag = v.magnitude();
        assert_eq!(mag, 5.0);
    }
    #[test]
    fn test_vector2_normalize() {
        let v = Vector2::new(3.0, 4.0);
        let norm = v.normalize();
        assert!((norm.magnitude() - 1.0).abs() < 1e-12);
    }
    #[test]
    fn test_vector2_distance() {
        let p1 = Vector2::new(1.0, 2.0);        
        let p2 = Vector2::new(4.0, 6.0);
        let dist = p1.metric_distance(&p2);
        assert_eq!(dist, 5.0);
    }
}

/// Type alias for 3D Matrices
pub type Matrix3 = nalgebra::Matrix3<f64>;


// Test module for Matrix3D
#[cfg(test)]
mod matrix_tests {
    use super::*;
    #[test]
    fn test_matrix_operations() {
        let m1 = Matrix3::new(1.0, 2.0, 3.0,
                              4.0, 5.0, 6.0,
                              7.0, 8.0, 9.0);
        let m2 = Matrix3::new(9.0, 8.0, 7.0,
                              6.0, 5.0, 4.0,
                              3.0, 2.0, 1.0);
        let m3 = m1 + m2;
        assert_eq!(m3, Matrix3::new(10.0, 10.0, 10.0,
                                     10.0, 10.0, 10.0,
                                     10.0, 10.0, 10.0));
        let m4 = m2 - m1;
        assert_eq!(m4, Matrix3::new(8.0, 6.0, 4.0,
                                     2.0, 0.0, -2.0,
                                     -4.0, -6.0, -8.0));
        let m5 = m1 * 2.0;
        assert_eq!(m5, Matrix3::new(2.0, 4.0, 6.0,
                                     8.0, 10.0, 12.0,
                                     14.0, 16.0, 18.0));
        let m6 = m2 / 2.0;
        assert_eq!(m6, Matrix3::new(4.5, 4.0, 3.5,
                                     3.0, 2.5, 2.0,
                                     1.5, 1.0, 0.5));
    }
    #[test]
    fn test_matrix_multiplication() {
        let m1 = Matrix3::new(1.0, 2.0, 3.0,
                              0.0, 1.0, 4.0,
                              5.0, 6.0, 0.0);   
        let m2 = Matrix3::new(1.0, 0.0, 0.0,
                              0.0, 1.0, 0.0,
                              0.0, 0.0, 1.0);
        let m3 = m1 * m2;
        assert_eq!(m3, Matrix3::new(1.0, 2.0, 3.0,
                                     0.0, 1.0, 4.0,
                                     5.0, 6.0, 0.0));
    }   
    #[test]
    fn test_transpose() {
        let m = Matrix3::new(1.0, 2.0, 3.0,
                             4.0, 5.0, 6.0,
                             7.0, 8.0, 9.0);
        let mt = m.transpose();
        assert_eq!(mt, Matrix3::new(1.0, 4.0, 7.0,
                                     2.0, 5.0, 8.0,
                                     3.0, 6.0, 9.0));
    }
    #[test]
    fn test_determinant() {
        let m = Matrix3::new(1.0, 2.0, 3.0,
                             0.0, 1.0, 4.0,
                             5.0, 6.0, 0.0);
        let det = m.determinant();
        assert_eq!(det, 1.0);
    }
    #[test]
    fn test_diagonal_terms() {
        let m = Matrix3::new(1.0, 2.0, 3.0,
                             4.0, 5.0, 6.0,
                             7.0, 8.0, 9.0);
        let diag = m.diagonal();
        assert_eq!(diag, Vector3::new(1.0, 5.0, 9.0));
    }
    #[test]
    fn test_build_isotropic_matrix() {
        let iso = Matrix3::from_diagonal_element(3.0);
        assert_eq!(iso, Matrix3::new(3.0, 0.0, 0.0,
                                      0.0, 3.0, 0.0,
                                      0.0, 0.0, 3.0));
   }
}   