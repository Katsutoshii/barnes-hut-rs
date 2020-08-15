//! Module for defining bodies (planets, etc.)
use plotters::prelude::*;

pub struct Body2D {
    pub pos: [f32; 2],
    pub mass: f32,
    pub color: RGBColor,
}
