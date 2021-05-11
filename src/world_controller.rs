use piston::input::GenericEvent;

use crate::world::World;


pub struct WorldController {
    pub world: World,
}


impl WorldController {
    pub fn new(world: World) -> WorldController {
        WorldController {
            world: world
        }
    }

    fn update_environment() {

    }

    fn update_colony() {

    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(_) = e.update_args() {
            self.update_environment();
            self.update_colony();
        }
    }    
}