use ndarray::{Array, Dim};
use ndarray_rand::{RandomExt, rand_distr::Distribution};
use ndarray_rand::rand_distr::{Normal, Uniform};


struct NES {
    pub learning_rate: f32,
    pub sigma: f32, 
    pub population_size: usize,
}


impl NES {
    pub fn new(learning_rate: f32, sigma: f32, population_size: usize) -> NES {
        NES {
            learning_rate,
            sigma,
            population_size,
        }
    }

    pub fn step() {
        
    }
}