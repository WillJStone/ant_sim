use glm::{Vec2, rotate_vec2};
use rand;


pub fn random_unit_vector() -> Vec2 {
    let mut v = Vec2::new(rand::random::<f32>(), rand::random::<f32>());
    v = glm::normalize(&v);

    return v
}


pub fn random_rotation(vec: &Vec2) -> Vec2 {
    // Take a value, in radians, between 0 and 1, subtract 0.5 so its now in the range
    // [-0.5, 0.5), divide by 5. So the new direction is +/ 0.1 radians
    // from the direction of the input vector
    let mut random_radians = rand::random::<f32>();
    random_radians -= 0.5;
    random_radians /= 2.0;

    let new_vec = rotate_vec2(vec, random_radians);

    return new_vec
}