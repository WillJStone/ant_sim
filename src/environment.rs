const SIZE: usize = 100;


#[derive(Clone, Copy)]
pub struct Cell {
    pub coordinates: [usize; 2],
    pub home_pheromone_concentration: f64,
    pub food_pheromone_concentration: f64,
    pub contains_ant: bool,
    pub contains_food: bool,
    pub is_nest: bool,
}


pub struct Environment {
    pub diffusion_rate: f64,
    pub size: usize,
    pub grid: [[Cell; SIZE]; SIZE],
}


impl Cell {
    fn new(coordinates: [usize; 2]) -> Cell {
        Cell {
            coordinates: coordinates,
            home_pheromone_concentration: 0.0,
            food_pheromone_concentration: 0.0,
            contains_ant: false,
            contains_food: false,
            is_nest: false,
        }
    }
}


impl Environment {
    pub fn new(diffusion_rate: f64) -> Environment {
        let mut grid = [[Cell::new([0, 0]); SIZE]; SIZE];
        for i in 0..SIZE {
            for j in 0..SIZE {
                grid[i][j].coordinates = [i, j];
            }
        }
        Environment {
            diffusion_rate: diffusion_rate,
            size: SIZE,
            grid: grid,
        }
    }

    pub fn perceive_surroundings(&self, index: [usize; 2]) -> Vec<Cell> {
        let i = index[0];
        let j = index[1];
        let can_go_left = if (i as i32 - 1) < 0 {false} else {true};
        let can_go_down = if (j as i32 - 1) < 0 {false} else {true};
        let can_go_right = if (i + 1) >= self.size {false} else {true};
        let can_go_up = if (j + 1) >= self.size {false} else {true};

        let mut surroundings: Vec<Cell> = Vec::new();

        if can_go_left {
            surroundings.push(self.grid[i - 1][j]);
        }
        if can_go_up {
            surroundings.push(self.grid[i][j + 1]);
        }
        if can_go_down {
            surroundings.push(self.grid[i][j - 1]);
        }
        if can_go_right {
            surroundings.push(self.grid[i + 1][j]);
        }
        if can_go_left && can_go_up { 
            surroundings.push(self.grid[i - 1][j + 1]);
        }
        if can_go_left && can_go_down {
            surroundings.push(self.grid[i - 1][j - 1]);
        }
        if can_go_right && can_go_up {
            surroundings.push(self.grid[i + 1][j + 1]);
        }
        if can_go_right && can_go_down {
            surroundings.push(self.grid[i + 1][j - 1]);
        }        

        surroundings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_environment() {
        let environment = Environment::new(0.9);
        assert_eq!(environment.grid[1][1].coordinates, [1, 1]);
    }
}
