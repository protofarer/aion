#[derive(Copy, Clone, Debug, Default)]
pub struct Pt {
    x: i32,
    y: i32,
}

impl Pt {
    pub const fn new(x: i32, y: i32) -> Pt {
        Pt { x, y }
    }
}

impl core::ops::Add for Pt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl core::ops::Mul for Pt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Rect {
    p1: Pt,
    p2: Pt,
}

impl Rect {
    pub const fn new(p1: Pt, p2: Pt) -> Rect {
        Rect { p1, p2 }
    }
}


pub fn rotate_point(x: f32, y: f32, rotation: f32, cx: f32, cy: f32) -> (f32, f32) {
    let x_translated = x - cx as f32;
    let y_translated = y - cy as f32;
    let x_rotated = x_translated * rotation.cos() + y_translated * rotation.sin();
    let y_rotated = x_translated * rotation.sin() - y_translated * rotation.cos();
    (x_rotated + cx as f32, y_rotated + cy as f32)
}