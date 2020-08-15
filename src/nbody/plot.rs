//! Module for plotting.

use plotters::prelude::*;
use crate::nbody::body::Body2D;

/// Creates a plot.
pub fn create_plot(filename: &str, width: u32, height: u32, bodies: &[Body2D]) ->
        Result<(), Box<dyn std::error::Error>> {

    println!("Starting plot ({width}, {height}) at {filename}",
        width=width,
        height=height,
        filename=filename);
    let root = BitMapBackend::new(filename, (width, height)).into_drawing_area();
    root.fill(&BLACK)?;
    for body in bodies {
        // Draw an circle on the drawing area
        root.draw(&Circle::new(
            (body.pos[0] as i32, body.pos[1] as i32),
            body.mass as i32,
            Into::<ShapeStyle>::into(&body.color).filled(),
        ))?;
    }

    Ok(())
}
