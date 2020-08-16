//! Module for plotting.

use plotters::prelude::*;
use std::process::Command;
use crate::nbody::simulation::NBodySimulation2D;

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

/// Compiles an MP4 from a directory of pngs.
/// Example file format: `"data/frames/img%04d.png"`
pub fn compile_mp4(filename_format: &str) {
    Command::new("ffmpeg")
        .args(&[
            "-y",
            "-r 30",
            "-s 500x500",
            &*format!("-i {}", filename_format),
            "-vcodec libx264",
            "-crf 4",
            "-pix_fmt yuv420p",
            "data/videos/sim.mp4"
        ])
        .output()
        .expect("Failed to execute process");
}
