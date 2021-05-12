use piston::input::GenericEvent;
use rand;
use rand::seq::SliceRandom;

use crate::Colony;
use crate::Environment;


pub struct WorldController {
    pub colony: Colony,
    pub environment: Environment,
}


impl WorldController {
    pub fn new(colony: Colony, environment: Environment) -> WorldController {
        WorldController {
            colony: colony,
            environment: environment,
        }
    }

    fn update_environment(&mut self) {
        for grid_row in self.environment.grid.iter_mut() {
            for grid_cell in grid_row.iter_mut() {
                grid_cell.home_pheromone_concentration *= self.environment.diffusion_rate;
                grid_cell.food_pheromone_concentration *= self.environment.diffusion_rate;
            }
        }
    }

    fn update_colony(&mut self) {
        // Move every ant randomly for now
        for ant in self.colony.ants.iter_mut() {
            let surroundings = self.environment.perceive_surroundings(ant.location);
            let new_cell = surroundings.choose(&mut rand::thread_rng()).unwrap();
            ant.location = new_cell.coordinates;
        }
    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(_) = e.update_args() {
            self.update_environment();
            self.update_colony();
        }
    }    
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn update_environment() {
        let mut world_controller = WorldController::new(Colony::new(10), Environment::new(100, 0.9));
        world_controller.environment.grid[0][0].home_pheromone_concentration = 1.0;
        world_controller.update_environment();
        assert_eq!(world_controller.environment.grid[0][0].home_pheromone_concentration, 0.9);
    }

    #[test]
    fn update_colony() {
        let mut world_controller = WorldController::new(Colony::new(10), Environment::new(100, 0.9));
        world_controller.update_colony();
        assert_ne!(world_controller.colony.ants[0].location, [0, 0]);
    }
}
