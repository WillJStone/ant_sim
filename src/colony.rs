use glm::Vec2;
use na::Point2;
use piston::input::GenericEvent;

use crate::environment::Environment;
use crate::utils::{random_unit_vector, random_rotation};



#[derive(Clone, Copy)]
pub struct Ant {
    pub coordinates: Point2<f32>,
    pub direction: Vec2,
    pub velocity: f32,
    pub grid_location: [usize; 2],
    pub has_food: bool
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
            grid_location: [1; 2],
            has_food: false,
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

    fn update(&mut self, environment: &mut Environment) {
        self.update_position(environment);
        self.direction = random_rotation(&self.direction);
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
                ant.update(environment);
            }
        }
    }
}