#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    #[inline]
    #[allow(non_snake_case)]
    pub const fn RGB(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 0xff }
    }
    #[inline]
    #[allow(non_snake_case)]
    pub const fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
    pub fn invert(self) -> Color {
        Color::RGBA(255 - self.r, 255 - self.g, 255 - self.b, 255 - self.a)
    }
    #[inline]
    pub const fn tuple_rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }
    #[inline]
    pub const fn tuple_rgba(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }
    #[inline]
    pub const fn rgba(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
    #[inline]
    pub const fn rgb(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
    pub const WHITE: Color = Color::RGBA(255, 255, 255, 255);
    pub const BLACK: Color = Color::RGBA(0, 0, 0, 255);
    pub const GRAY: Color = Color::RGBA(128, 128, 128, 255);
    pub const GREY: Color = Color::GRAY;
    pub const RED: Color = Color::RGBA(255, 0, 0, 255);
    pub const GREEN: Color = Color::RGBA(0, 255, 0, 255);
    pub const BLUE: Color = Color::RGBA(0, 0, 255, 255);
    pub const MAGENTA: Color = Color::RGBA(255, 0, 255, 255);
    pub const YELLOW: Color = Color::RGBA(255, 255, 0, 255);
    pub const CYAN: Color = Color::RGBA(0, 255, 255, 255);
    // pub const WHITE: [u8; 4] = Color::RGBA(255, 255, 255, 255).rgba();
    // pub const BLACK: [u8; 4] = Color::RGBA(0, 0, 0, 255).rgba();
    // pub const GRAY: [u8; 4] = Color::RGBA(128, 128, 128, 255).rgba();
    // pub const GREY: [u8; 4] = Color::GRAY;
    // pub const RED: [u8; 4] = Color::RGBA(255, 0, 0, 255).rgba();
    // pub const GREEN: [u8; 4] = Color::RGBA(0, 255, 0, 255).rgba();
    // pub const BLUE: [u8; 4] = Color::RGBA(0, 0, 255, 255).rgba();
    // pub const MAGENTA: [u8; 4] = Color::RGBA(255, 0, 255, 255).rgba();
    // pub const YELLOW: [u8; 4] = Color::RGBA(255, 255, 0, 255).rgba();
    // pub const CYAN: [u8; 4] = Color::RGBA(0, 255, 255, 255).rgba();
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Color {
        Color::RGB(r, g, b)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Color {
        Color::RGBA(r, g, b, a)
    }
}
