//! Executable to run the simulation locally in a Piston GUI window.
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

mod vector;
mod nbody;
mod quadtree;

use vector::{
    Scalar,
    Vector,
    Vector3D};
use nbody::{
    generate_galaxy,
    generate_blackhole,
    nbody_barnes_hut,
    MovingBody3D,
    NBodyConfig3D,
    NBodySimulation3D
};
use quadtree::{
    MassQuadtree, BoundingBox2D
};


/// Width of stars in the GUI
const STAR_WIDTH: f64 = 2.;
const BLACKHOLE_WIDTH: f64 = 18.;
const EVENTHORIZON_WIDTH: f64 = 20.;
const BLACK: [f32; 4] = [0., 0., 0., 0.0];
const SOLID_BLACK: [f32; 4] = [0., 0., 0., 1.0];
const RED1: [f32; 4] = [1., 0., 0., 0.1];
const WHITE7: [f32; 4] = [1., 1., 1., 0.7];

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
        let star_square = rectangle::square(0.0, 0.0, STAR_WIDTH);
        let blackhole_square = rectangle::square(0.0, 0.0, BLACKHOLE_WIDTH);
        let eventhorizon_square = rectangle::square(0.0, 0.0, EVENTHORIZON_WIDTH);
        let sim = &mut self.sim;
        let quadtree: MassQuadtree = MassQuadtree::new(&sim.r, &sim.m, BoundingBox2D { 
            min_x: 0.,
            max_x: 500.,
            min_y: 0.,
            max_y: 500.});

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            // Plot all black holes
            for i in 0..sim.config.num_blackholes {
                let transform = c
                    .transform
                    .trans(sim.r[i].x as f64, sim.r[i].y as f64)
                    .trans(-EVENTHORIZON_WIDTH / 2., -EVENTHORIZON_WIDTH / 2.);
                ellipse(RED1, eventhorizon_square, transform, gl);
                let transform_center = c
                    .transform
                    .trans(sim.r[i].x as f64, sim.r[i].y as f64)
                    .trans(-BLACKHOLE_WIDTH / 2., -BLACKHOLE_WIDTH / 2.);
                ellipse(SOLID_BLACK, blackhole_square, transform_center, gl);
            }
            // Plot all non-blackholes
            for i in sim.config.num_blackholes..sim.n {
                let transform = c
                    .transform
                    .trans(sim.r[i].x as f64, sim.r[i].y as f64)
                    .trans(-STAR_WIDTH / 2., -STAR_WIDTH / 2.);
                ellipse(WHITE7, star_square, transform, gl);
            }
        });
    }

    /// Updates the simulation for one timestep.
    fn update(&mut self, _args: &UpdateArgs) {
        // nbody_direct(self.sim, 0.1);
        nbody_barnes_hut(self.sim, 0.1, 2.);
    }

    fn click(&mut self, mouse_xy: &[f64; 2]) {
        let (x, y) = (mouse_xy[0], mouse_xy[1]);
        println!("Pressed mouse button ({}, {})", x, y);
        // Center of galaxy.
        let c: MovingBody3D = MovingBody3D {
            r: Vector3D::from_xy(x as Scalar, y as Scalar),
            v: Vector3D::zero(),
            m: 5e6,
        };
        generate_blackhole(self.sim, &c);
    }
}

/// Main routine
fn main() {
    // Init the simulation
    let min_dist: Scalar = 10.;
    let min_r: Vector3D = Vector3D::from_xy(0., 0.);
    let max_r: Vector3D = Vector3D::from_xy(500., 500.,);
    let config = NBodyConfig3D::new(min_dist, min_r, max_r);
    let mut sim: NBodySimulation3D = NBodySimulation3D::empty(6, config);
    
    // Center of galaxy.
    let c: MovingBody3D = MovingBody3D {
        r: Vector3D::from_xy(250., 250.),
        v: Vector3D::zero(),
        m: 5e6,
    };
    generate_galaxy(&mut sim, &c);

    nbody_barnes_hut(&mut sim, 0.1, 2.);
    nbody_barnes_hut(&mut sim, 0.1, 2.);
    nbody_barnes_hut(&mut sim, 0.1, 2.);
    return;

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("galaxy", [max_r.x as u32, max_r.y as u32])
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
    let mut mouse_xy = [0., 0.];
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Mouse(_button)) = e.press_args() {
            app.click(&mouse_xy);
        }

        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        
        // Track the mouse position
        e.mouse_cursor(|pos| {
            mouse_xy = pos
        });
    }
}
