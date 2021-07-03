use core::num;

use crate::simulation::arena::Arena;
use crate::simulation::colony::Ant;

pub struct Environment {
    pub arena: Arena,
    pub colony: Vec<Ant>,
}


impl Environment {
    pub fn new(arena_size: usize, diffusion_rate: f64, num_ants: usize) -> Environment {
        let arena = Arena::new(arena_size, diffusion_rate);
        let colony = vec![Ant::new(); num_ants];

        Environment {
            arena,
            colony,
        }
    }
}