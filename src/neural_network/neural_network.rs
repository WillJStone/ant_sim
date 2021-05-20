use ndarray::{Array, Dim};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;


struct Layer {
    w: Array<f32, Dim<[usize; 2]>>,
    b: Array<f32, Dim<[usize; 2]>>,
}


impl Layer {
    fn new(num_input: usize, num_output: usize) -> Layer {
        let distribution = Uniform::new(-0.5, 0.5);
        let w = Array::random((num_input, num_output), distribution);
        let b = Array::random((1, num_output), distribution);

        Layer {
            w: w,
            b: b,
        }
    }

    fn forward(&self, input: Array<f32, Dim<[usize; 2]>>) -> Array<f32, Dim<[usize; 2]>> {
        let h = input.dot(&self.w) + &self.b;

        h
    }
}