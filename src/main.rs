extern crate piston;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate nalgebra as na;
extern crate nalgebra_glm as glm;


use piston::window::WindowSettings;
use piston::event_loop::{Events, EventLoop, EventSettings};
use piston::input::RenderEvent;
use glutin_window::GlutinWindow;
use opengl_graphics::{OpenGL, GlGraphics};


pub use crate::colony::Colony;
pub use crate::environment::Environment;
// pub use crate::world_controller::WorldController;
pub use crate::world_view::{WorldView, WorldViewSettings};

mod colony;
mod environment;
mod world_view;
mod utils;


const NUM_ANTS: usize = 200;
const ARENA_SIZE: usize = 100;
const DIFFUSION_RATE: f64 = 0.9;
const PIXEL_SIZE: usize = 4;
const RESOLUTION: usize = PIXEL_SIZE * ARENA_SIZE;


fn main() {
    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Ant Colony Simulation", [RESOLUTION as f64; 2])
        .graphics_api(opengl)
        .exit_on_esc(true);

    let mut window: GlutinWindow = settings.build()
        .expect("Could not create window");
    let event_settings: EventSettings = EventSettings::new()
        .max_fps(50)
        .ups(50);

    let mut events = Events::new(event_settings);
    let mut gl = GlGraphics::new(opengl);

    let mut colony = Colony::new(NUM_ANTS);
    let mut environment = Environment::new(ARENA_SIZE, DIFFUSION_RATE);
    let world_view = WorldView::new(WorldViewSettings::new());

    while let Some(e) = events.next(&mut window) {
        environment.update(&e);
        colony.update(&mut environment, &e);
        
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                use graphics::{clear};

                clear([0.0; 4], g);
                world_view.draw(&environment, &colony, &c, g);
            });
        }
    }
}