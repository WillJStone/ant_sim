use ndarray::{Array, Dim};
use rand;

use crate::simulation::arena::{Cell, Arena};
use crate::simulation::utils::{get_direction_from_coords, random_unit_vector, random_rotation, rotate_array2};



#[derive(Clone)]
pub struct Ant {
    pub coordinates: Array<f32, Dim<[usize; 1]>>,
    pub direction: Array<f32, Dim<[usize; 1]>>,
    pub velocity: f32,
    pub max_perception_distance: f32,
    pub field_of_view: f32,
    pub grid_location: [usize; 2],
    pub has_food: bool,
    num_samples: usize,
}


impl Ant {
    pub fn new() -> Ant {
        Ant {
            coordinates: Array::from(vec![1.0, 1.0]),
            direction: random_unit_vector(),
            velocity: 0.5,
            max_perception_distance: 10.0,
            field_of_view: std::f32::consts::PI / 1.0,
            grid_location: [1; 2],
            has_food: false,
            num_samples: 5,
        }
    }

    fn update_position(&mut self, environment: &Arena) {
        let mut new_coordinates: Array<f32, Dim<[usize; 1]>>;
        let mut new_grid_cell_indices: [usize; 2];
        let mut i = 0;
        while i < 10 {
            new_coordinates = &self.coordinates + &self.direction * self.velocity;
            new_grid_cell_indices = [new_coordinates[[0]] as usize, new_coordinates[[1]] as usize];

            // If the cell is traversable, go there, if not do a random rotation on the direction and try again
            if environment.cell_is_traversable(new_grid_cell_indices) {
                self.coordinates = new_coordinates;
                self.grid_location = new_grid_cell_indices;
                break;
            } else {
                self.direction = random_rotation(&self.direction, 2. * std::f32::consts::PI);
            }
            i += 1;
        }
    }

    fn perceive_surroundings(&self, environment: &Arena) -> Vec<Cell> {
        let mut surroundings: Vec<Cell> = Vec::new();
        // Take 10 samples of the surroundings, maybe make the sample size tunable in future
        while surroundings.len() < self.num_samples {
            let random_direction = random_rotation(&self.direction, self.field_of_view);
            let random_distance = rand::random::<f32>() * self.max_perception_distance;
            let sample_point = self.coordinates.clone() + random_direction * random_distance;
            match environment.get_cell_from_point(&sample_point) {
                Ok(cell) => surroundings.push(cell),
                Err(_) => continue,
            };
        }

        surroundings
    }

    fn _perceive_surroundings(&self, environment: &Arena) -> Vec<Cell> {
        // Really dumb implementation of just looking at the local neighborhood of cells
        let mut surroundings: Vec<Cell> = Vec::new();
        for i in (self.grid_location[0] as i32 - 2)..(self.grid_location[0] + 3) as i32 {
            for j in (self.grid_location[1] as i32)..((self.grid_location[1] + 3) as i32) {
                let cell: Cell;
                if i < 0 || i >= environment.size as i32|| j < 0 || j >= environment.size as i32 {
                    cell = Cell::new([0; 2]);
                } else {
                    cell = environment.grid[i as usize][j as usize].clone();
                }
                surroundings.push(cell);
            }
        }

        surroundings
    }

    fn get_feature_vector(&self, environment: &Arena) -> Array<f32, Dim<[usize; 2]>> {
        let surroundings = self.perceive_surroundings(environment);
        let mut feature_vec: Vec<f32> = Vec::new();

        // First 3 channels are the ant's personal info
        feature_vec.push(self.has_food as i32 as f32);
        feature_vec.push(self.direction[[0]]);
        feature_vec.push(self.direction[[1]]);
        
        for cell in surroundings.iter() {
            let coords_as_array = Array::from(self.coordinates.clone());
            let direction_to_cell = get_direction_from_coords(&self.coordinates, &coords_as_array);
            feature_vec.push(cell.is_nest as i32 as f32); // Have to go through int to get to f32 from bool
            feature_vec.push(cell.is_traversable as i32 as f32);
            feature_vec.push(cell.food_amount as f32);
            feature_vec.push(cell.food_pheromone_concentration as f32);
            feature_vec.push(cell.nest_pheromone_concentration as f32);
            feature_vec.push(direction_to_cell[[0]]);
            feature_vec.push(direction_to_cell[[1]]);
        }

        Array::from_shape_vec((1, feature_vec.len()), feature_vec).unwrap()
    }

    pub fn step(&mut self, action: Array<f32, Dim<[usize; 1]>>, arena: &mut Arena) -> (Array<f32, Dim<[usize; 2]>>, f32) {
        self.direction = rotate_array2(&self.direction, action[0]);
        self.update_position(arena);

        let mut reward: f32 = 0.;

        if arena.cell_has_food(self.grid_location) && !self.has_food {
            arena.take_food(self.grid_location);
            reward += 0.1;
            self.has_food = true;
        }

        if arena.cell_is_nest(self.grid_location) && self.has_food {
            reward += 1.;
            self.has_food = false;
        }

        if self.has_food {
            arena.place_food_pheromone(self.grid_location);
        } else {
            arena.place_nest_pheromone(self.grid_location);
        }

        let observation = self.get_feature_vector(arena);

        (observation, reward)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_perceive_surroundings() {
        let environment = Arena::new(50, 0.99);
        let ant = Ant::new();
        let surroundings = ant.perceive_surroundings(&environment);
        
        assert_eq!(surroundings.len(), 5);
    }
    #[test]
    fn test_ant_get_feature_vector() {
        let mut environment = Arena::new(50, 0.99);
        let ant = Ant::new();
        let feature_vector = ant.get_feature_vector(&mut environment);

        assert_eq!(feature_vector.len(), 38);
    }
}