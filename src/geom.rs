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
