pub struct Ant {
    pub movement_speed: u8,
    pub perception_distance: u8,
    pub has_food: bool
}


impl Ant {
    pub fn new() -> Ant {
        Ant {
            movement_speed: 1,
            perception_distance: 3,
            has_food: false,
        }
    }
}