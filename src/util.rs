pub fn rotate_point(x: f32, y: f32, rotation: f32, cx: f32, cy: f32) -> (f32, f32) {
    let x_translated = x - cx as f32;
    let y_translated = y - cy as f32;
    let x_rotated = x_translated * rotation.cos() + y_translated * rotation.sin();
    let y_rotated = x_translated * rotation.sin() - y_translated * rotation.cos();
    (x_rotated + cx as f32, y_rotated + cy as f32)
}

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * nalgebra_glm::pi::<f32>() / 180.0
}

pub fn change_heading(heading: f32, deg: f32) -> f32 {
    (heading + deg_to_rad(deg)) % nalgebra_glm::two_pi::<f32>()
}
