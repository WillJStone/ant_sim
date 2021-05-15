use piston::input::GenericEvent;


#[derive(Clone, Copy)]
pub struct Cell {
    pub coordinates: [usize; 2],
    pub nest_pheromone_concentration: f64,
    pub food_pheromone_concentration: f64,
    pub food_amount: f64,
    pub is_nest: bool,
    pub is_traversable: bool,
}


pub struct Environment {
    pub diffusion_rate: f64,
    pub size: usize,
    pub grid: Vec<Vec<Cell>>,
}


impl Cell {
    fn new(coordinates: [usize; 2]) -> Cell {
        Cell {
            coordinates: coordinates,
            nest_pheromone_concentration: 0.0,
            food_pheromone_concentration: 0.0,
            food_amount: 0.0,
            is_nest: false,
            is_traversable: true,
        }
    }

    pub fn set_nest_pheromone_concentration(&mut self) {
        self.nest_pheromone_concentration = 1.0;
    }

    pub fn set_food_pheromone_concentration(&mut self) {
        self.food_pheromone_concentration = 1.0;
    }

    fn update(&mut self, diffusion_rate: f64) {
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
        self.grid[index[0]][index[1]].nest_pheromone_concentration = 1.0;
    }

    pub fn place_food_pheromone(&mut self, index: [usize; 2]) {
        self.grid[index[0]][index[1]].food_pheromone_concentration = 1.0;
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

    pub fn update<E: GenericEvent>(&mut self, e: &E) {
        if let Some(_) = e.update_args() {
            for grid_row in self.grid.iter_mut() {
                for cell in grid_row.iter_mut() {
                    cell.update(self.diffusion_rate);
                }
            }
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
    // #[test]
    // fn update_environment() {
    //     let mut environment = Environment::new(100, 0.9);
    //     environment.grid[0][0].nest_pheromone_concentration = 1.0;
    //     environment.update();
    //     assert_eq!(environment.grid[0][0].nest_pheromone_concentration, 0.9);
    // }
