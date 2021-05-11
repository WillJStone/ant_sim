use piston::input::GenericEvent;

use crate::World;


const DIFFUSION_RATE: f64 = 0.9;


pub struct WorldController {
    pub world: World,
}


impl WorldController {
    pub fn new(world: World) -> WorldController {
        WorldController {
            world: world
        }
    }

    fn update_environment(&mut self) {
        for grid_row in self.world.environment.grid.iter_mut() {
            for grid_cell in grid_row.iter_mut() {
                grid_cell.home_pheromone_concentration *= DIFFUSION_RATE;
                grid_cell.food_pheromone_concentration *= DIFFUSION_RATE;
            }
        }
    }

    fn update_colony(&self) {

    }

    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(_) = e.update_args() {
            self.update_environment();
            self.update_colony();
        }
    }    
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn update_environment() {
        let mut world_controller = WorldController::new(World::new());
        world_controller.world.environment.grid[0][0].home_pheromone_concentration = 1.0;
        world_controller.update_environment();
        assert_eq!(world_controller.world.environment.grid[0][0].home_pheromone_concentration, 0.9);
    }
}