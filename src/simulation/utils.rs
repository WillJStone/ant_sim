use ndarray::{Array, Dim};
use rand;


pub fn normalize_array(array: Array<f32, Dim<[usize; 1]>>) -> Array<f32, Dim<[usize; 1]>> {
    let sum_of_squares: f32 = array
        .iter()
        .map(|x| f32::powf(*x, 2.))
        .sum();

    array / sum_of_squares.sqrt()
}


pub fn random_unit_vector() -> Array<f32, Dim<[usize; 1]>> {
    let v = Array::from(vec![rand::random::<f32>(), rand::random::<f32>()]);
    
    normalize_array(v)
}


pub fn rotate_array2(array: &Array<f32, Dim<[usize; 1]>>, angle: f32) -> Array<f32, Dim<[usize; 1]>> {
    let x = array[[0]] * angle.cos() - array[[1]] * angle.sin();
    let y = array[[0]] * angle.sin() + array[[1]] * angle.cos();

    Array::from(vec![x, y])
}


pub fn get_direction_from_coords(
    point_1: &Array<f32, Dim<[usize; 1]>>, 
    point_2: &Array<f32, Dim<[usize; 1]>>
) -> Array<f32, Dim<[usize; 1]>> {
    let vec = point_2 - point_1;

    normalize_array(vec)
}


pub fn random_rotation(array: &Array<f32, Dim<[usize; 1]>>, range: f32) -> Array<f32, Dim<[usize; 1]>> {
    // Take a value, in radians, between 0 and 1, subtract 0.5 so its now in the range
    // [-0.5, 0.5), divide by 2. So the new direction is +/ 0.25 radians
    // from the direction of the input vector
    let mut random_radians = rand::random::<f32>();
    random_radians -= 0.5;
    random_radians *= range;

    let new_vec = rotate_array2(array, random_radians);

    return new_vec
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_random_unit_vector() {
        let r1 = random_unit_vector();
        let r2 = random_unit_vector();

        assert_ne!(r1, r2);
    }
}