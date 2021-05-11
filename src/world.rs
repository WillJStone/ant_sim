use crate::colony::Colony;
use crate::environment::Environment;


pub struct World {
    colony: Colony,
    environment: Environment,
}


impl World {
    pub fn new() -> World {
        World {
            colony: Colony::new(),
            environment: Environment::new(),
        }
    }
}
