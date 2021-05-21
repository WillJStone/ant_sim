use ndarray::{Array, Dim};

pub fn relu(array: Array<f32, Dim<[usize; 2]>>) -> Array<f32, Dim<[usize; 2]>> {
    fn _relu(x: f32) -> f32 {
        if x > 0.0 {
            return x
        } else {
            return 0.0
        }
    }

    array.mapv_into(|x| _relu(x))
}