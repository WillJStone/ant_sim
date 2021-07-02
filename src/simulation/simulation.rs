use std::f64;

use crate::simulation::colony::Colony;
use crate::simulation::arena::Arena;
use crate::neural_network::mlp::MLP;


pub struct Simulation {
    pub arena: Arena,
    pub colony: Colony,
}


pub struct SimulationResult {
    pub num_iters: usize,
    pub food_returned_to_nest: f64,
    pub food_remaining: f64,
    pub proportion_explored: f64
}


impl Simulation {
    pub fn new(arena_size: usize, diffusion_rate: f64, num_ants: usize, decision_network: MLP) -> Simulation {
        let arena = Arena::new(arena_size, diffusion_rate);
        let colony = Colony::new(num_ants, decision_network);

        Simulation {
            arena,
            colony,
        }
    }

    pub fn run(&mut self, num_steps: usize) -> SimulationResult {
        let mut i = 0;
        while i < num_steps {
            self.arena.update();
            self.colony.update(&mut self.arena);
            i += 1;
        };

        SimulationResult::new(
            i, 
            self.arena.food_returned_to_nest,
            self.arena.total_food_remaining(),
            self.arena.num_cells_visited() as f64/ self.arena.size.pow(2) as f64 
        )
    }
}


impl SimulationResult {
    pub fn new(num_iters: usize, food_returned_to_nest: f64, food_remaining: f64, proportion_explored: f64) -> SimulationResult {
        SimulationResult {
            num_iters, 
            food_returned_to_nest,
            food_remaining,
            proportion_explored
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simulation_new() {
        let decision_network: MLP = MLP::new(37, vec![16, 1]);
        let simulation = Simulation::new(50, 0.99, 100, decision_network);
        assert_eq!(simulation.arena.size, 50);
    }

    #[test]
    fn test_simulation_run() {
        let decision_network: MLP = MLP::new(38, vec![16, 1]);
        let mut simulation = Simulation::new(50, 0.99, 100, decision_network);
        let _sim_result = simulation.run(10);
        // let fake_sim_result = SimulationResult::new(10, 0., 25.);

        // assert_eq!(sim_result.num_iters, fake_sim_result.num_iters);
    }
}