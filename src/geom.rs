pub fn rotate_point(x: f32, y: f32, rotation: f32, cx: f32, cy: f32) -> (f32, f32) {
    let x_translated = x - cx as f32;
    let y_translated = y - cy as f32;
    let x_rotated = x_translated * rotation.cos() + y_translated * rotation.sin();
    let y_rotated = x_translated * rotation.sin() - y_translated * rotation.cos();
    (x_rotated + cx as f32, y_rotated + cy as f32)
}
