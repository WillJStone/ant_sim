use core::num;

use crate::simulation::colony::Colony;
use crate::simulation::environment::Environment;


pub struct Simulation {
    pub environment: Environment,
    pub colony: Colony,
}


impl Simulation {
    pub fn new(arena_size: usize, diffusion_rate: f64, num_ants: usize) -> Simulation {
        let environment = Environment::new(arena_size, diffusion_rate);
        let colony = Colony::new(num_ants);

        Simulation {
            environment,
            colony,
        }
    }

    pub fn run(&mut self, num_steps: usize) {
        let mut i = 0;
        while i < num_steps {
            self.environment.update();
            self.colony.update(&mut self.environment);
            i += 1;
        };
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simulation_new() {
        let mut simulation = Simulation::new(50, 0.99, 100);
        simulation.run(1);
    }
}