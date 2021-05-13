use graphics::types::Color;
use graphics::{Context, Graphics};

use crate::world_controller::WorldController;


pub struct WorldViewSettings {
    pub ant_color: Color,
    pub pixel_size: usize
}


pub struct WorldView {
    pub settings: WorldViewSettings,
}


impl WorldViewSettings {
    pub fn new() -> WorldViewSettings {
        WorldViewSettings {
            ant_color: [1.0, 0.0, 0.0, 1.0],
            pixel_size: 4,
        }
    }
}


impl WorldView {
    pub fn new(settings: WorldViewSettings) -> WorldView {
        WorldView {
            settings: settings,
        }
    }

    fn draw_environment<G: Graphics>(&self, controller: &WorldController, c: &Context, g: &mut G) {
        use graphics::{Rectangle, rectangle};
        let nest_cell = Rectangle::new([0.0, 0.0, 1.0, 0.5]);

        for (i, grid_row) in controller.environment.grid.iter().enumerate() {
            for (j, cell) in grid_row.iter().enumerate() {
                let square = rectangle::square(
                    (i * self.settings.pixel_size) as f64, 
                    (j * self.settings.pixel_size) as f64, 
                    self.settings.pixel_size as f64
                );
                if cell.is_nest {
                    nest_cell.draw(square, &c.draw_state, c.transform, g);
                }

                if cell.food_amount > 0.0 {
                    Rectangle::new([0.0, 1.0, 0.0, cell.food_amount as f32])
                        .draw(square, &c.draw_state, c.transform, g);
                }
            }
        }
    }

    fn draw_colony<G: Graphics>(&self, controller: &WorldController, c: &Context, g: &mut G) {
        use graphics::{Rectangle, rectangle};
        let ref settings = self.settings;
        let ant_vis = Rectangle::new(self.settings.ant_color);

        for ant in controller.colony.ants.iter() {
            let square = rectangle::square(
                (ant.location[0] * settings.pixel_size) as f64, 
                (ant.location[1] * settings.pixel_size) as f64, 
                self.settings.pixel_size as f64
            );

            ant_vis.draw(square, &c.draw_state, c.transform, g);
        }
    }

    pub fn draw<G: Graphics>(&self, controller: &WorldController, c: &Context, g: &mut G) {
        self.draw_environment(controller, c, g);
        self.draw_colony(controller, c, g);

        
    }
}