use piston::input::GenericEvent;
use rand;
use rand::seq::SliceRandom;

use crate::Colony;
use crate::environment::{Cell, Environment};


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
                grid_cell.nest_pheromone_concentration *= self.environment.diffusion_rate;
                grid_cell.food_pheromone_concentration *= self.environment.diffusion_rate;
            }
        }
    }

    fn update_colony(&mut self) {
        // Move every ant randomly for now
        for ant in self.colony.ants.iter_mut() {
            let surroundings = self.environment.perceive_surroundings(ant.location);
            let traversable_cells: Vec<&Cell> = surroundings.iter().filter(|&cell| cell.is_traversable).collect();

            let max_food_cell = traversable_cells
                .iter()
                .max_by(|c1, c2| c1.food_amount.partial_cmp(&c2.food_amount).unwrap())
                .unwrap();

            if max_food_cell.food_amount > 0.0 {
                self.environment.grid[max_food_cell.coordinates[0]][max_food_cell.coordinates[1]].food_amount -= 0.1;
                ant.has_food = true;
            }

            if ant.has_food {
                self.environment.place_food_pheromone(ant.location);
                let max_nest_cell = traversable_cells
                    .iter()
                    .max_by(|c1, c2| c1.nest_pheromone_concentration.partial_cmp(&c2.nest_pheromone_concentration).unwrap())
                    .unwrap();

                ant.location = max_nest_cell.coordinates;
            } else {
                self.environment.place_nest_pheromone(ant.location);
                let max_food_cell = traversable_cells
                    .iter()
                    .max_by(|c1, c2| c1.food_pheromone_concentration.partial_cmp(&c2.food_pheromone_concentration).unwrap())
                    .unwrap();
                let min_nest_cell = traversable_cells
                    .iter()
                    .min_by(|c1, c2| c1.nest_pheromone_concentration.partial_cmp(&c2.nest_pheromone_concentration).unwrap())
                    .unwrap();

                if max_food_cell.food_pheromone_concentration == 0.0 {
                    if rand::random::<f64>() > 0.5 {
                        ant.location = min_nest_cell.coordinates;
                    } else {
                        let random_traversable_cell = traversable_cells.choose(&mut rand::thread_rng()).unwrap();
                        ant.location = random_traversable_cell.coordinates;
                    }
                } else {
                    ant.location = max_food_cell.coordinates;
                }
            }

            // let new_cell = traversable_cells.choose(&mut rand::thread_rng()).unwrap();
            //ant.location = new_cell.coordinates;
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
        world_controller.environment.grid[0][0].nest_pheromone_concentration = 1.0;
        world_controller.update_environment();
        assert_eq!(world_controller.environment.grid[0][0].nest_pheromone_concentration, 0.9);
    }

    #[test]
    fn update_colony() {
        let mut world_controller = WorldController::new(Colony::new(10), Environment::new(100, 0.9));
        world_controller.update_colony();
        assert_ne!(world_controller.colony.ants[0].location, [0, 0]);
    }
}
