//! Executable to run the simulation locally in a Piston GUI window.
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod vector;
mod nbody;
mod quadtree;

use vector::{Scalar};
use nbody::{
    generate_galaxy, maintain_bounds, nbody_barnes_hut,
    NBodySimulation3D, HEIGHT, WIDTH,
};

/// Width of stars in the GUI
const STAR_WIDTH: f64 = 2.;

/// Piston App for GUI
pub struct App<'a> {
    gl: GlGraphics,                 // OpenGL drawing backend.
    sim: &'a mut NBodySimulation3D, // The simulation
}

/// Implementation of Piston App
impl App<'_> {
    /// Renders a frame of the simulation
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [Scalar; 4] = [0.0, 0.0, 0.0, 0.0];
        const WHITE7: [Scalar; 4] = [1.0, 1.0, 1.0, 0.7];

        let square = rectangle::square(0.0, 0.0, STAR_WIDTH);
        let sim = &mut self.sim;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Plot all points
            for i in 1..sim.n {
                let transform = c
                    .transform
                    .trans(sim.r[i].x as f64, sim.r[i].y as f64)
                    .trans(-25.0, -25.0);
                ellipse(WHITE7, square, transform, gl);
            }
        });
    }

    /// Updates the simulation for one timestep.
    fn update(&mut self, _args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // nbody_direct(self.sim, 0.1);
        nbody_barnes_hut(self.sim, 0.1, 10.);
        maintain_bounds(self.sim);
    }
}

/// Main routine
fn main() {
    // Init the simulation
    let mut sim: NBodySimulation3D = generate_galaxy(1000);

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("galaxy", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        sim: &mut sim,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}
