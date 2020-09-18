//! Defines a splitable bounding box
use crate::vector::Scalar;

/// Splitable bounding box in 2 dimensions.
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox2D {
    pub min_x: Scalar,
    pub max_x: Scalar,
    pub min_y: Scalar,
    pub max_y: Scalar
}

/// implementation for a splitable bounding box in 2 dimensions.
impl BoundingBox2D {
    /// Gets the center X position of the bounding box.
    pub fn cx(&self) -> Scalar {
        (self.max_x + self.min_x) / 2.
    }

    /// Gets the center Y position of the bounding box.
    pub fn cy(&self) -> Scalar {
        (self.max_y + self.min_y) / 2.
    }

    /// Gets the width of this bounding box (x direction).
    pub fn width(&self) -> Scalar {
        self.max_x - self.min_x
    }

    // Returns the quadtrant of a point
    pub fn quadrant(&self, x: Scalar, y: Scalar) -> usize {
        let x_bit = (x >= self.cx()) as usize;
        let y_bit = (y >= self.cy()) as usize;
        x_bit + (y_bit << 1)
    }

    /// Gets the subquadtrant of this bounding box.
    /// The quadtrant number must be between 0 and 3.
    /// The LSB represents left (0) or right (1) in the x direction.
    /// The MSB represents left (0) or right (1) in the y direction.
    pub fn child(&self, quadrant: usize) -> Self {
        match quadrant {
            0b00 => Self {
                min_x: self.min_x,
                max_x: self.cx(),
                min_y: self.min_y,
                max_y: self.cy()
            },
            0b01 => Self {
                min_x: self.cx(),
                max_x: self.max_x,
                min_y: self.min_y,
                max_y: self.cy()
            },
            0b10 => Self {
                min_x: self.min_x,
                max_x: self.cx(),
                min_y: self.cy(),
                max_y: self.max_y
            },
            0b11 => Self {
                min_x: self.cx(),
                max_x: self.max_x,
                min_y: self.cy(),
                max_y: self.max_y
            },
            _ => Self {
                min_x: self.min_x,
                max_x: self.max_x,
                min_y: self.min_y,
                max_y: self.max_y
            },
        }
    }
}
