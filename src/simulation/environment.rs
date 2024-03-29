use ndarray::{Array, Dim};
use piston::input::GenericEvent;


#[derive(Clone, Copy)]
pub struct Cell {
    pub coordinates: [usize; 2],
    pub nest_pheromone_concentration: f64,
    pub food_pheromone_concentration: f64,
    pub food_amount: f64,
    pub is_nest: bool,
    pub is_traversable: bool,
    pub visited: bool,
}


pub struct Environment {
    pub diffusion_rate: f64,
    pub size: usize,
    pub grid: Vec<Vec<Cell>>,
    pub food_returned_to_nest: f64,
}


impl Cell {
    pub fn new(coordinates: [usize; 2]) -> Cell {
        Cell {
            coordinates: coordinates,
            nest_pheromone_concentration: 0.0,
            food_pheromone_concentration: 0.0,
            food_amount: 0.0,
            is_nest: false,
            is_traversable: true,
            visited: false,
        }
    }

    pub fn set_nest_pheromone_concentration(&mut self) {
        self.nest_pheromone_concentration = 1.0;
    }

    pub fn set_food_pheromone_concentration(&mut self) {
        self.food_pheromone_concentration = 1.0;
    }

    pub fn get_continuous_location(&self) -> Array<f32, Dim<[usize; 1]>> {
        Array::from(vec![self.coordinates[0] as f32, self.coordinates[1] as f32])
    }

    fn update(&mut self, diffusion_rate: f64) {
        if self.nest_pheromone_concentration < 0.01 {
            self.nest_pheromone_concentration = 0.0;
        }
        if self.food_pheromone_concentration < 0.01 {
            self.food_pheromone_concentration = 0.0;
        }
        self.nest_pheromone_concentration *= diffusion_rate;
        self.food_pheromone_concentration *= diffusion_rate;
    }
}


impl Environment {
    pub fn new(arena_size: usize, diffusion_rate: f64) -> Environment {
        let mut grid = vec![vec![Cell::new([0, 0]); arena_size]; arena_size];
        for i in 0..arena_size {
            for j in 0..arena_size {
                grid[i][j].coordinates = [i, j];
            }
        }
        let mut environment = Environment {
            diffusion_rate: diffusion_rate,
            size: arena_size,
            grid: grid,
            food_returned_to_nest: 0.
        };
        environment.pad_edges();
        environment.set_nest_area();
        environment.place_food();

        environment
    }

    fn pad_edges(&mut self) {
        for i in 0..self.size {
            self.grid[i][0].is_traversable = false;
            self.grid[i][self.size -1].is_traversable = false;
            self.grid[0][i].is_traversable = false;
            self.grid[self.size -1][i].is_traversable = false;
        }
    }

    fn set_nest_area(&mut self) {
        for i in 1..6 {
            for j in 1..6 {
                self.grid[i][j].is_nest = true;
            }
        }
    }

    fn place_food(&mut self) {
        for i in (self.size - 6)..(self.size - 1)  {
            for j in (self.size - 6)..(self.size - 1) {
                self.grid[i][j].food_amount = 1.0;
            }
        }
    }

    pub fn place_nest_pheromone(&mut self, index: [usize; 2]) {
        if self.grid[index[0]][index[1]].nest_pheromone_concentration > 0.9 {
            self.grid[index[0]][index[1]].nest_pheromone_concentration = 1.0;
        } else {
            self.grid[index[0]][index[1]].nest_pheromone_concentration += 0.01;
        }
    }

    pub fn place_food_pheromone(&mut self, index: [usize; 2]) {
        if self.grid[index[0]][index[1]].food_pheromone_concentration > 0.9 {
            self.grid[index[0]][index[1]].food_pheromone_concentration = 1.0;
        } else {
            self.grid[index[0]][index[1]].food_pheromone_concentration += 0.01;
        }
    }

    pub fn cell_has_food(&self, index: [usize; 2]) -> bool {
        let cell = self.grid[index[0]][index[1]];
        if cell.food_amount > 0.0 {
            return true
        }

        false
    }

    pub fn cell_is_traversable(&self, index: [usize; 2]) -> bool {
        if self.grid[index[0]][index[1]].is_traversable {
            return true
        }

        false
    }

    pub fn cell_is_nest(&self, index: [usize; 2]) -> bool {
        if self.grid[index[0]][index[1]].is_nest {
            return true
        }

        false
    }

    pub fn take_food(&mut self, index: [usize; 2]) {
        self.grid[index[0]][index[1]].food_amount -= 0.1;
    }

    pub fn get_cell_from_point(&self, point: &Array<f32, Dim<[usize; 1]>>) -> Result<Cell, &str> {
        let x_coord = point[[0]] as i32;
        let y_coord = point[[1]] as i32;
        if x_coord < 0 || x_coord >= self.size as i32 || y_coord < 0 || y_coord >= self.size as i32 {
            return Err("out of bounds")
        }

        Ok(self.grid[x_coord as usize][y_coord as usize])
    }

    pub fn perceive_surroundings(&self, index: [usize; 2]) -> Vec<Cell> {
        let mut surroundings: Vec<Cell> = Vec::new();
        for i in (index[0] - 1)..(index[0] + 2) {
            for j in (index[1] -1)..(index[1] + 2) {
                surroundings.push(self.grid[i][j]);
            }
        }

        surroundings
    }

    pub fn total_food_remaining(&self) -> f64 {
        let mut total_food_amount = 0.0;
        for grid_row in self.grid.iter() {
            for cell in grid_row.iter() {
                total_food_amount += cell.food_amount;
            }
        }

        total_food_amount
    }

    pub fn num_cells_visited(&self) -> i32 {
        let mut num_cells = 0;
        for grid_row in self.grid.iter() {
            for cell in grid_row.iter() {
                num_cells += cell.visited as i32;
            }
        }

        num_cells
    }

    pub fn set_cell_as_visited(&mut self, index: [usize; 2]) {
        self.grid[index[0]][index[1]].visited = true;
    }

    pub fn update(&mut self) {
        for grid_row in self.grid.iter_mut() {
            for cell in grid_row.iter_mut() {
                cell.update(self.diffusion_rate);
            }
        }
    }

    pub fn update_piston<E: GenericEvent>(&mut self, e: &E) {
        if let Some(_) = e.update_args() {
            self.update();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_environment() {
        let environment = Environment::new(100, 0.9);
        assert_eq!(environment.grid[1][1].coordinates, [1, 1]);
    }
}
