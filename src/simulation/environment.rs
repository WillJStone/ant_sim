use ndarray::{Array, Dim};

use crate::simulation::arena::Arena;
use crate::simulation::colony::Ant;

pub struct Environment {
    arena: Arena,
    colony: Vec<Ant>,
}


pub struct StepResult { 
    pub observations: Vec<Array<f32, Dim<[usize; 1]>>>,
    pub rewards: Vec<f32>,
    pub done: bool,
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

    pub fn reset(&mut self) {
        let arena = Arena::new(self.arena.size, self.arena.diffusion_rate);
        let colony = vec![Ant::new(); self.colony.len()];

        self.arena = arena;
        self.colony = colony;
    }
}