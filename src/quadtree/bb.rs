//! Defines a splitable bounding box
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox2D {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32
}

impl BoundingBox2D {
    pub fn cx(&self) -> f32 {
        (self.max_x + self.min_x) / 2.
    }

    pub fn cy(&self) -> f32 {
        (self.max_y + self.min_y) / 2.
    }

    pub fn width(&self) -> f32 {
        self.max_x - self.min_x
    }

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
