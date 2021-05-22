use ndarray::{Array, Dim};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

use crate::neural_network::utils::{relu, reshape_array};


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

    fn from_existing_weights(weights: Array<f32, Dim<[usize; 2]>>, bias: Array<f32, Dim<[usize; 2]>>) -> Layer {
        Layer {
            w: weights, 
            b: bias,
        }
    }

    fn forward(&self, input: Array<f32, Dim<[usize; 2]>>) -> Array<f32, Dim<[usize; 2]>> {
        let h = input.dot(&self.w) + &self.b;

        relu(h)
    }
}


struct MLP {
    layers: Vec<Layer>,
}


impl MLP {
    pub fn new(input_dimension: usize, hidden_sizes: Vec<usize>) -> MLP {
        // NOTE: The last element of the hidden_sizes vector is the output size
        let mut layers: Vec<Layer> = Vec::new();
        let mut num_input = input_dimension;
        for i in 0..hidden_sizes.len() {
            let layer = Layer::new(num_input, hidden_sizes[i]);
            layers.push(layer);
            num_input = hidden_sizes[i];
        }

        MLP { layers }
    }

    pub fn from_flattened_weights(
        input_dimension: usize, 
        hidden_sizes: Vec<usize>, 
        weight_array: Array<f32, Dim<[usize; 1]>>) -> MLP {
            let mut layers: Vec<Layer> = Vec::new();
            let mut num_input = input_dimension;
            let mut current_array_index: usize = 0;
            for i in 0..hidden_sizes.len() {
                let w_end = current_array_index + num_input * hidden_sizes[i];

                let array_slice = weight_array.slice(s![current_array_index..w_end]);
                let w = reshape_array(array_slice.to_owned(), [num_input, hidden_sizes[i]]);
                current_array_index = w_end;

                let b_end = current_array_index + hidden_sizes[i];
                let array_slice = weight_array.slice(s![current_array_index..b_end]);
                let b = reshape_array(array_slice.to_owned(), [1, hidden_sizes[i]]);
                current_array_index = b_end;

                num_input = hidden_sizes[i];

                layers.push(Layer::from_existing_weights(w, b));
            }

            MLP { layers }
        }

    pub fn flatten_weights(&self) -> Array<f32, Dim<[usize; 1]>> {
        let mut flattened_weights: Vec<f32> = Vec::new();
        for layer in self.layers.iter() {
            for w in layer.w.iter() {
                flattened_weights.push(*w)
            }

            for b in layer.b.iter() {
                flattened_weights.push(*b);
            }
        }

        Array::from(flattened_weights)
    }

    pub fn forward(&self, input: Array<f32, Dim<[usize; 2]>>) -> Array<f32, Dim<[usize; 2]>> {
        let mut layer_result = input;
        for layer in self.layers.iter() {
            layer_result = layer.forward(layer_result);
        }

        layer_result
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_layer() {
        let layer = Layer::new(8, 4);
        assert_eq!(layer.w.len(), 32);
        assert_eq!(layer.b.len(), 4);
    }

    #[test]
    fn test_layer_from_existing_weights() {
        let distribution = Uniform::new(-0.5, 0.5);
        let w = Array::random((8, 4), distribution);
        let b = Array::random((1, 4), distribution);

        let layer = Layer::from_existing_weights(w.clone(), b.clone());

        assert_eq!(layer.w, w);
        assert_eq!(layer.b, b);

    }

    #[test]
    fn test_layer_forward() {
        let w: Array<f32, Dim<[usize; 2]>> = Array::eye(4);
        let b: Array<f32, Dim<[usize; 2]>> = Array::zeros((1, 4));
        let sample_input = Array::ones((1, 4));
        let layer = Layer::from_existing_weights(w, b);

        let out = layer.forward(sample_input.clone());

        assert_eq!(sample_input, out);
    }

    #[test]
    fn test_mlp_new() {
        let mlp = MLP::new(4, vec![2, 2]);

        assert_eq!(mlp.layers.len(), 2);
        assert_eq!(mlp.layers[0].w.len(), 8);
    }

    #[test]
    fn test_mlp_from_flattened_weights() {
        let mlp = MLP::new(4, vec![2, 2]);
        let flattened_weights = mlp.flatten_weights();
        let reconstructed_mlp = MLP::from_flattened_weights(4, vec![2, 2], flattened_weights);

        assert_eq!(mlp.layers[0].w, reconstructed_mlp.layers[0].w);
    }

    #[test]
    fn test_mlp_forward() {
        let mlp = MLP::new(4, vec![2, 2]);
        let sample_input = Array::ones((1, 4));
        let sample_output = mlp.forward(sample_input);

        assert_eq!(sample_output.len(), 2);
    }

    #[test]
    fn test_flatten_weights() {
        let mlp = MLP::new(4, vec![2, 2]);
        let flat_weights = mlp.flatten_weights();
        assert_eq!(flat_weights.len(), 16);
    }
}
