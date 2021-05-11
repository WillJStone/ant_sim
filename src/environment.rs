const SIZE: usize = 100;


#[derive(Clone, Copy)]
pub struct Cell {
    pub home_pheromone_concentration: f64,
    pub food_pheromone_concentration: f64,
    pub contains_ant: bool,
    pub contains_food: bool,
    pub is_nest: bool,
}


pub struct Environment {
    pub size: usize,
    pub grid: [[Cell; SIZE]; SIZE],
}


impl Cell {
    fn new() -> Cell {
        Cell {
            home_pheromone_concentration: 0.0,
            food_pheromone_concentration: 0.0,
            contains_ant: false,
            contains_food: false,
            is_nest: false,
        }
    }
}


impl Environment {
    pub fn new() -> Environment {
        Environment {
            size: SIZE,
            grid: [[Cell::new(); SIZE]; SIZE],
        }
    }
}