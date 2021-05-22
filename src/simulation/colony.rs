use glm::Vec2;
use na::Point2;
use piston::input::GenericEvent;
use rand;
use std;

use crate::simulation::environment::{Cell, Environment};
use crate::utils::{random_unit_vector, random_rotation};



#[derive(Clone, Copy)]
pub struct Ant {
    pub coordinates: Point2<f32>,
    pub direction: Vec2,
    pub velocity: f32,
    pub max_perception_distance: f32,
    pub field_of_view: f32,
    pub grid_location: [usize; 2],
    pub has_food: bool,
    num_samples: usize,
}


pub struct Colony {
    pub ants: Vec<Ant>,
}


impl Ant {
    pub fn new() -> Ant {
        Ant {
            coordinates: Point2::new(1.0, 1.0),
            direction: random_unit_vector(),
            velocity: 0.5,
            max_perception_distance: 10.0,
            field_of_view: std::f32::consts::PI / 1.0,
            grid_location: [1; 2],
            has_food: false,
            num_samples: 5,
        }
    }

    fn update_position(&mut self, environment: &Environment) {
        let mut new_coordinates = self.coordinates + self.direction * self.velocity;
        let mut new_grid_cell_indices = [new_coordinates.x as usize, new_coordinates.y as usize];

        // If this move runs up against a non-traversable cell, turn around and make another move
        if !environment.cell_is_traversable(new_grid_cell_indices) {
            self.direction *= -1.0;
            new_coordinates = self.coordinates + self.direction * self.velocity;
            new_grid_cell_indices = [new_coordinates.x as usize, new_coordinates.y as usize];
        }
        
        self.coordinates = new_coordinates;
        self.grid_location = new_grid_cell_indices;
    }

    fn perceive_surroundings(&self, environment: &Environment) -> Vec<Cell> {
        let mut surroundings: Vec<Cell> = Vec::new();
        // Take 10 samples of the surroundings, maybe make the sample size tunable in future
        while surroundings.len() <= self.num_samples {
            let random_direction = random_rotation(&self.direction, self.field_of_view);
            let random_distance = rand::random::<f32>() * self.max_perception_distance;
            let sample_point = self.coordinates + random_direction * random_distance;
            match environment.get_cell_from_point(sample_point) {
                Ok(cell) => surroundings.push(cell),
                Err(_) => continue,
            };
        }

        surroundings
    }

    fn update_direction(&mut self, surroundings: Vec<Cell>) {
        if self.has_food {
            // If the ant is holding food, go towards the nest if it's visible, otherwise try
            // to follow the nest pheromone if any is detectable
            if surroundings.iter().any(|c| c.is_nest) {
                let nest_cell = surroundings.iter().find(|c| c.is_nest).unwrap();
                let nest_cell_point = nest_cell.get_continuous_location();
                let point_difference = nest_cell_point - self.coordinates;
                let new_direction = Vec2::new(point_difference.x, point_difference.y);
                self.direction = glm::normalize(&new_direction);

            } else if surroundings.iter().any(|c| c.nest_pheromone_concentration > 0.0) {
                let max_pheromone_cell = surroundings.iter().max_by(
                    |c1, c2| c1.nest_pheromone_concentration.partial_cmp(&c2.nest_pheromone_concentration).unwrap()
                ).unwrap();
                let max_cell_point = max_pheromone_cell.get_continuous_location();
                let point_difference = max_cell_point - self.coordinates;
                let new_direction = Vec2::new(point_difference.x, point_difference.y);
                self.direction = glm::normalize(&new_direction);

            } else {
                self.direction = random_rotation(&self.direction, 0.5);
            }
        } else {
            // If the ant does not have food, go towards food directly if it's visible, otherwise
            // try to follow the food pheromone if any is detectable
            if surroundings.iter().any(|c| c.food_amount > 0.0) {
                let food_cell = surroundings.iter().find(|c| c.food_amount > 0.0).unwrap();
                let food_cell_point = food_cell.get_continuous_location();
                let point_difference = food_cell_point - self.coordinates;
                let new_direction = Vec2::new(point_difference.x, point_difference.y);
                self.direction = glm::normalize(&new_direction);

            } else if surroundings.iter().any(|c| c.food_pheromone_concentration > 0.0) {
                let max_pheromone_cell = surroundings
                    .iter()
                    .filter(|c| c.food_pheromone_concentration > 0.0)
                    .max_by(
                        |c1, c2| c1.food_pheromone_concentration.partial_cmp(&c2.food_pheromone_concentration).unwrap()
                    ).unwrap();
                let max_cell_point = max_pheromone_cell.get_continuous_location();
                let point_difference = max_cell_point - self.coordinates;
                let new_direction = Vec2::new(point_difference.x, point_difference.y);
                self.direction = glm::normalize(&new_direction);
            } else {
                self.direction = random_rotation(&self.direction, 0.5);
            }
        }
    }

    fn update(&mut self, environment: &mut Environment) {
        self.update_position(environment);
        if environment.cell_has_food(self.grid_location) && !self.has_food {
            environment.take_food(self.grid_location);
            self.direction *= -1.0;
            self.has_food = true;
        }

        if environment.cell_is_nest(self.grid_location) && self.has_food {
            self.direction *= -1.0;
            self.has_food = false;
        }

        if self.has_food {
            environment.place_food_pheromone(self.grid_location);
        } else {
            environment.place_nest_pheromone(self.grid_location);
        }

        let surroundings = self.perceive_surroundings(environment);
        self.update_direction(surroundings);
    }
}


impl Colony {
    pub fn new(num_ants: usize) -> Colony {
        Colony {
            ants: vec![Ant::new(); num_ants],
        }
    }

    pub fn update(&mut self, environment: &mut Environment) {
        for ant in self.ants.iter_mut() {
            ant.update(environment);
        }
    }

    pub fn update_piston<E: GenericEvent>(&mut self, environment: &mut Environment, e: &E) {
        // Move every ant randomly for now
        if let Some(_) = e.update_args() {
            self.update(environment);
        }
    }
}