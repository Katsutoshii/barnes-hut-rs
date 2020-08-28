extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

mod nbody;
mod quadtree;

use nbody::{
    Body2D,
    load_bodies_2d,
    NBodySimulation2D,
    nbody_direct_2d,
    nbody_barnes_hut_2d,
    generate_galaxy,
    maintain_bounds,
    HEIGHT,
    WIDTH,
    CENTER};

const STAR_WIDTH: f64 = 2.;

pub struct App<'a> {
    gl: GlGraphics,                 // OpenGL drawing backend.
    sim: &'a mut NBodySimulation2D,    // The simulation
}

impl App<'_> {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
        const WHITE7: [f32; 4] = [1.0, 1.0, 1.0, 0.7];

        let square = rectangle::square(0.0, 0.0, STAR_WIDTH);
        let sim = &mut self.sim; 

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Plot all points
            for i in 1..sim.n {
                let transform = c
                    .transform
                    .trans(sim.rx[i] as f64, sim.ry[i] as f64)
                    .trans(-25.0, -25.0);
                ellipse(WHITE7, square, transform, gl);
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        // nbody_direct_2d(self.sim, 0.1);
        nbody_barnes_hut_2d(self.sim, 0.1, 10.);
        maintain_bounds(self.sim);
    }
}

fn main() {
    // Init the simulation
    let mut sim: NBodySimulation2D = generate_galaxy(1000);

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("galaxy", [500, 500])
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
