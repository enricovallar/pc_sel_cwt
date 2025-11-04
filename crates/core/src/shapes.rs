/// Represents the physical geometry of a single hole.
#[derive(Debug, Clone, PartialEq)]
pub enum HoleShape {
    Circle { radius: f64 },
    Square {
        side: f64,
    },
    Rectangle {
        width: f64,
        height: f64,
    },
}

// Tests for the geometry module
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hole_shape_creation() {
        let circle = HoleShape::Circle { radius: 0.1 };
        assert_eq!(circle, HoleShape::Circle { radius: 0.1 });
    }

    #[test]
    fn test_hole_shape_equality() {
        let square1 = HoleShape::Square { side: 0.2 };
        let square2 = HoleShape::Square { side: 0.2 };
        assert_eq!(square1, square2);
    }

    #[test]
    fn test_hole_shape_inequality() {
        let rect = HoleShape::Rectangle { width: 0.3, height: 0.4 };
        let circle = HoleShape::Circle { radius: 0.3 };
        assert_ne!(rect, circle);
    }
}