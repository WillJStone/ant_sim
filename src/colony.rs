extern crate nalgebra as na;
extern crate nalgebra_glm as glm;

use glm::Vec2;
use na::Point2;
use piston::input::GenericEvent;
use rand;
use rand::seq::SliceRandom;

use crate::environment::{Cell, Environment};


#[derive(Clone, Copy)]
pub struct Ant {
    pub coordinates: Point2<f32>,
    pub direction: Vec2,
    pub grid_location: [usize; 2],
    pub has_food: bool
}


pub struct Colony {
    pub ants: Vec<Ant>,
}


impl Ant {
    pub fn new() -> Ant {
        let mut direction = Vec2::new(rand::random::<f32>(), rand::random::<f32>());
        direction = glm::normalize(&direction);
        Ant {
            coordinates: Point2::new(1.0, 1.0),
            direction: direction,
            grid_location: [1; 2],
            has_food: false,
        }
    }
}


impl Colony {
    pub fn new(num_ants: usize) -> Colony {
        Colony {
            ants: vec![Ant::new(); num_ants],
        }
    }

    pub fn update<E: GenericEvent>(&mut self, environment: &mut Environment, e: &E) {
        // Move every ant randomly for now
        if let Some(_) = e.update_args() {
            for ant in self.ants.iter_mut() {
                let surroundings = environment.perceive_surroundings(ant.grid_location);
                let traversable_cells: Vec<&Cell> = surroundings.iter().filter(|&cell| cell.is_traversable).collect();
    
                let max_food_cell = traversable_cells
                    .iter()
                    .max_by(|c1, c2| c1.food_amount.partial_cmp(&c2.food_amount).unwrap())
                    .unwrap();
    
                if max_food_cell.food_amount > 0.0 {
                    environment.grid[max_food_cell.coordinates[0]][max_food_cell.coordinates[1]].food_amount -= 0.1;
                    ant.has_food = true;
                }
    
                if ant.has_food {
                    environment.place_food_pheromone(ant.grid_location);
                    let max_nest_cell = traversable_cells
                        .iter()
                        .max_by(|c1, c2| c1.nest_pheromone_concentration.partial_cmp(&c2.nest_pheromone_concentration).unwrap())
                        .unwrap();
    
                    ant.grid_location = max_nest_cell.coordinates;
                } else {
                    environment.place_nest_pheromone(ant.grid_location);
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
                            ant.grid_location = min_nest_cell.coordinates;
                        } else {
                            let random_traversable_cell = traversable_cells.choose(&mut rand::thread_rng()).unwrap();
                            ant.grid_location = random_traversable_cell.coordinates;
                        }
                    } else {
                        ant.grid_location = max_food_cell.coordinates;
                    }
                }
            }
        }
    }
}