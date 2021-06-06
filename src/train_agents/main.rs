extern crate rust_es;
extern crate lib;

use ndarray::{Array, Dim};
use ndarray_npy::write_npy;

use rust_es::nes::NES;
use rust_es::objective::Objective;
use rust_es::utils::random_gaussian_vector;
use lib::simulation::simulation::Simulation;
use lib::neural_network::mlp::MLP;


const NUM_ANTS: usize = 50;
const ARENA_SIZE: usize = 25;
const DIFFUSION_RATE: f64 = 0.99;
//const NUM_PERCEPTION_SAMPLES: usize = 10;
const NUM_STEPS_PER_SIM: usize = 500;

#[derive(Clone)]
struct SimulationWrapper {
}


impl SimulationWrapper {

}


impl Objective for SimulationWrapper {
    fn call(&self, x: &Array<f32, Dim<[usize; 1]>>) -> f32 {
        let x = x.clone();
        let mlp = MLP::from_flattened_weights(37, vec![16, 2], x);
        let mut simulation = Simulation::new(ARENA_SIZE, DIFFUSION_RATE, NUM_ANTS, mlp);
        let simulation_result = simulation.run(NUM_STEPS_PER_SIM);

        let reward = - 10. * simulation_result.food_returned_to_nest - simulation_result.proportion_explored + simulation_result.food_remaining / 10.;

        reward as f32
    }
}


fn main() {
    let mu = random_gaussian_vector(37*16*2, 0., 0.1);
    let sigma = Array::ones(37*16*2) / 10.;
    let callable = SimulationWrapper {};
    let mut nes = NES::new(callable.clone(), mu, sigma, 128, 0.01, 0.01, true);
    
    for i in 0..500 {
        nes.step();
        println!("Step {} complete", i);
    }

    write_npy("trial_4.npy", &nes.mu).unwrap();
}