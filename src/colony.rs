const NUM_ANTS: usize = 10;


#[derive(Clone, Copy)]
pub struct Ant {
    pub location: [usize; 2],
    pub movement_speed: u8,
    pub perception_distance: u8,
    pub has_food: bool
}


pub struct Colony {
    pub ants: [Ant; NUM_ANTS],
}


impl Ant {
    pub fn new() -> Ant {
        Ant {
            location: [0; 2],
            movement_speed: 1,
            perception_distance: 3,
            has_food: false,
        }
    }
}


impl Colony {
    pub fn new() -> Colony {
        Colony {
            ants: [Ant::new(); NUM_ANTS],
        }
    }
}