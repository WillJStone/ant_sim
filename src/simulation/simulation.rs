use std::f64;

use crate::simulation::colony::Colony;
use crate::simulation::environment::Environment;
use crate::neural_network::neural_network::MLP;


pub struct Simulation {
    pub environment: Environment,
    pub colony: Colony,
}


pub struct SimulationResult {
    pub num_iters: usize,
    pub total_food_amount: f64,
}


impl Simulation {
    pub fn new(arena_size: usize, diffusion_rate: f64, num_ants: usize, decision_network: MLP) -> Simulation {
        let environment = Environment::new(arena_size, diffusion_rate);
        let colony = Colony::new(num_ants, decision_network);

        Simulation {
            environment,
            colony,
        }
    }

    pub fn run(&mut self, num_steps: usize) -> SimulationResult {
        let mut i = 0;
        while i < num_steps && self.environment.total_food_remaining() > 0.0 {
            self.environment.update();
            self.colony.update(&mut self.environment);
            i += 1;
        };

        SimulationResult::new(i, self.environment.total_food_remaining())
    }
}


impl SimulationResult {
    pub fn new(num_iters: usize, total_food_amount: f64) -> SimulationResult {
        SimulationResult {
            num_iters, 
            total_food_amount,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simulation_new() {
        let decision_network: MLP = MLP::new(37, vec![16, 2]);
        let simulation = Simulation::new(50, 0.99, 100, decision_network);
        assert_eq!(simulation.environment.size, 50);
    }

    #[test]
    fn test_simulation_run() {
        let decision_network: MLP = MLP::new(37, vec![16, 2]);
        let mut simulation = Simulation::new(50, 0.99, 100, decision_network);
        let sim_result = simulation.run(10);
        let fake_sim_result = SimulationResult::new(10, 25.0);

        assert_eq!(sim_result.num_iters, fake_sim_result.num_iters);
    }
}