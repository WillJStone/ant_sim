#[derive(Clone, Copy)]
pub struct Ant {
    pub location: [usize; 2],
    pub has_food: bool
}


pub struct Colony {
    pub ants: Vec<Ant>,
}


impl Ant {
    pub fn new() -> Ant {
        Ant {
            location: [0; 2],
            has_food: false,
        }
    }
}


impl Colony {
    pub fn new(num_ants: usize) -> Colony {
        Colony {
            ants: vec![Ant::new(); num_ants],
        }
    }
}