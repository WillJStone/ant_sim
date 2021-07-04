use ndarray::{Array, Dim};
use piston::input::GenericEvent;

use crate::simulation::arena::Arena;
use crate::simulation::ant::Ant;

pub struct Environment {
    pub arena: Arena,
    colony: Vec<Ant>,
}


pub struct StepResult { 
    pub observations: Vec<Array<f32, Dim<[usize; 2]>>>,
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

    pub fn step_piston<E: GenericEvent>(&mut self, actions: Vec<Array<f32, Dim<[usize; 1]>>>, e: &E) {
        if let Some(_) = e.update_args() {
            assert_eq!(actions.len(), self.colony.len());

            let mut observations: Vec<Array<f32, Dim<[usize; 2]>>> = Vec::new();
            let mut rewards: Vec<f32> = Vec::new();
            for i in 0..actions.len() { 
                let (observation, reward) = self.colony[i].step(actions[i].clone(), &mut self.arena);
                observations.push(observation);
                rewards.push(reward);
            }
        }
    }

    pub fn step(&mut self, actions: Vec<Array<f32, Dim<[usize; 1]>>>) -> StepResult {
        assert_eq!(actions.len(), self.colony.len());

        let mut observations: Vec<Array<f32, Dim<[usize; 2]>>> = Vec::new();
        let mut rewards: Vec<f32> = Vec::new();
        for i in 0..actions.len() { 
            let (observation, reward) = self.colony[i].step(actions[i].clone(), &mut self.arena);
            observations.push(observation);
            rewards.push(reward);
        }

        StepResult {
            observations,
            rewards,
            done: false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_step() {
        let num_ants = 50;
        let mut environment = Environment::new(50, 0.99, num_ants);
        let actions = vec![Array::zeros(1); num_ants];
        let result = environment.step(actions);

        assert_eq!(result.observations.len(), num_ants);
        assert_eq!(result.rewards.len(), num_ants);
    }
}