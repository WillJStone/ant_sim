use crate::colony::Colony;
use crate::environment::Environment;


pub struct World {
    pub colony: Colony,
    pub environment: Environment,
}


impl World {
    pub fn new() -> World {
        World {
            colony: Colony::new(),
            environment: Environment::new(),
        }
    }
}
