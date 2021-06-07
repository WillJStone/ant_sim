extern crate rust_es;
extern crate lib;

use ndarray::{Array, Dim};
use ndarray_npy::write_npy;

use rust_es::nes::NES;
use rust_es::objective::Objective;
use rust_es::utils::random_gaussian_vector;
use lib::simulation::simulation::Simulation;
use lib::neural_network::mlp::MLP;


const NUM_ANTS: usize = 1;
const ARENA_SIZE: usize = 16;
const DIFFUSION_RATE: f64 = 0.995;
//const NUM_PERCEPTION_SAMPLES: usize = 10;
const NUM_STEPS_PER_SIM: usize = 250;
const INPUT_DIMENSION: usize = 38;

#[derive(Clone)]
struct SimulationWrapper {
}


impl SimulationWrapper {

}


impl Objective for SimulationWrapper {
    fn call(&self, x: &Array<f32, Dim<[usize; 1]>>) -> f32 {
        let x = x.clone();
        let num_runs = 10;
        let mut total_reward = 0 as f64;
        for _ in 0..num_runs {
            let mlp = MLP::from_flattened_weights(INPUT_DIMENSION, vec![16, 2], x.clone());
            let mut simulation = Simulation::new(ARENA_SIZE, DIFFUSION_RATE, NUM_ANTS, mlp);
            let simulation_result = simulation.run(NUM_STEPS_PER_SIM);

            let reward = -simulation_result.food_returned_to_nest - simulation_result.proportion_explored + simulation_result.food_remaining / 10.;
            total_reward += reward;
        }
        

        total_reward as f32/ num_runs as f32
    }
}


fn main() {
    let hidden_sizes = vec![16, 2];
    let num_parameters = INPUT_DIMENSION * hidden_sizes[0] * hidden_sizes[1];
    let mu = random_gaussian_vector(num_parameters, 0., 1.);
    let sigma = Array::ones(num_parameters);
    let callable = SimulationWrapper {};
    let mut nes = NES::new(callable.clone(), mu, sigma, 32, 0.001, 0.001, true);
    
    for i in 0..500 {
        nes.step();
        println!("Step {} complete", i);
    }

    write_npy("trial_4.npy", &nes.mu).unwrap();
}