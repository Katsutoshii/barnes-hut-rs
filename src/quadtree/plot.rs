use plotters::prelude::*;

use crate::quadtree::{Quadtree};

const COLORS: [RGBColor; 5] = [WHITE, YELLOW, RED, BLUE, GREEN];

/// Creates a plot.
pub fn create_plot(
        filename: &str,
        width: u32,
        height: u32,
        sim: &NBodySimulation2D
    ) -> Result<(), Box<dyn std::error::Error>> {

    println!("Starting plot ({width}, {height}) at {filename}",
        width=width,
        height=height,
        filename=filename);
    let root = BitMapBackend::new(filename, (width, height)).into_drawing_area();
    root.fill(&BLACK)?;
    for i in 0..sim.n {
        root.draw(&Circle::new(
            (sim.x[i] as i32, sim.y[i] as i32),
            (sim.m[i].cbrt() / 20.0) as i32,
            Into::<ShapeStyle>::into(&COLORS[i % COLORS.len()]).filled(),
        ))?;
    }

    Ok(())
}
#[test]
