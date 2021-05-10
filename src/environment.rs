const SIZE: usize = 100;


pub struct Cell {
    pub home_pheromone_concentration: f64,
    pub food_pheromone_concentration: f64,
    pub contains_ant: bool,
    pub contains_food: bool,
    pub is_nest: bool,
}


pub struct Environment {
    pub world: [[Cell; SIZE]; Size],
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
            world: [[Cell::new(); Size]; Size],
        }
    }
}