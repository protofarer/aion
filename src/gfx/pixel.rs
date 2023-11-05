use log::{error, warn};
use rand::Rng;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Color([u8; 4]);

impl Color {
    const fn new(arr: [u8; 4]) -> Color {
        Color(arr)
    }
    #[inline]
    pub fn as_bytes(&self) -> &[u8; 4] {
        &self.0
    }
    #[inline]
    pub fn as_rgb(&self) -> [u8; 4] {
        self.0
    }
    #[inline]
    pub fn as_hsl(&self) -> [u8; 4] {
        // normalize
        let r = self.0[0] as f32 / 255.;
        let g = self.0[1] as f32 / 255.;
        let b = self.0[2] as f32 / 255.;

        // luminance
        let max_value = r.max(g.max(b));
        let min_value = r.min(g.min(b));
        let l = (max_value + min_value) / 2.;

        // saturation
        let delta = max_value - min_value;
        let s = if delta == 0. {
            0.
        } else {
            delta / (1. - (2. * l - 1.).abs())
        };

        // hue
        let h = if delta == 0.0 {
            0.0
        } else if max_value == r {
            ((g - b) / delta) % 6.0
        } else if max_value == g {
            (b - r) / delta + 2.0
        } else {
            (r - g) / delta + 4.0
        };
        let h = (h * 60.0).round();

        let h = (h % 360.).round() as u8;
        let s = (s * 100.).round() as u8;
        let l = (l * 100.).round() as u8;
        [h, s, l, self.0[3]]
    }
    #[inline]
    #[allow(non_snake_case)]
    pub const fn RGB(r: u8, g: u8, b: u8) -> Color {
        Color([r, g, b, 0xff])
    }
    #[inline]
    #[allow(non_snake_case)]
    pub const fn RGBA(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color([r, g, b, a])
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn HSL(h: f32, s: f32, l: f32) -> Color {
        let [r, g, b] = Self::calc_hsl(h, s, l);
        Color([r, g, b, 0xff])
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn HSLA(h: f32, s: f32, l: f32, a: f32) -> Color {
        let [r, g, b] = Self::calc_hsl(h, s, l);
        if a > 1. || a < 0. {
            warn!("Alpha out of range: {}", a);
        }
        let a = (a.clamp(0., 1.) * 255.).round() as u8;
        Color([r, g, b, a])
    }
    pub fn invert(self) -> Color {
        Color([
            255 - self.0[0],
            255 - self.0[1],
            255 - self.0[2],
            255 - self.0[3],
        ])
    }
    pub fn rng() -> Color {
        let mut rng = rand::thread_rng();
        let z = rng.gen::<f32>();
        let color_idx = (z * 8.0).floor() as i32; // number of prebuilt color constants
        match color_idx {
            0 => WHITE,
            1 => RED,
            2 => ORANGE,
            3 => YELLOW,
            4 => GREEN,
            5 => BLUE,
            6 => CYAN,
            7 => GRAY,
            _ => GRAY,
        }
    }
    fn calc_hsl(h: f32, s: f32, l: f32) -> [u8; 3] {
        // normalize
        let h = h % 360.;
        let s = s / 100.;
        let l = l / 100.;

        // chroma
        let c = (1. - (2. * l - 1.).abs()) * s;

        // intermediate val
        let x = c * (1. - ((h / 60.) % 2. - 1.).abs());

        let mut r1 = 0.;
        let mut g1 = 0.;
        let mut b1 = 0.;

        if h >= 0. && h < 60. {
            r1 = c;
            g1 = x;
        } else if h >= 60. && h < 120. {
            r1 = x;
            g1 = c;
        } else if h >= 120. && h < 180. {
            g1 = c;
            b1 = x;
        } else if h >= 180. && h < 240. {
            g1 = x;
            b1 = c;
        } else if h >= 240. && h < 300. {
            r1 = x;
            b1 = c;
        } else {
            r1 = c;
            b1 = x;
        }

        let m = l - c / 2.;

        let r = ((r1 + m) * 255.).round() as u8;
        let g = ((g1 + m) * 255.).round() as u8;
        let b = ((b1 + m) * 255.).round() as u8;
        [r, g, b]
    }
}
impl Default for Color {
    fn default() -> Self {
        WHITE
    }
}

pub const WHITE: Color = Color::new([255, 255, 255, 255]);
pub const BLACK: Color = Color::new([0, 0, 0, 255]);
pub const GRAY: Color = Color::new([128, 128, 128, 255]);
pub const RED: Color = Color::new([255, 0, 0, 255]);
pub const ORANGE: Color = Color::new([255, 165, 0, 255]);
pub const YELLOW: Color = Color::new([255, 255, 0, 255]);
pub const GREEN: Color = Color::new([0, 255, 0, 255]);
pub const BLUE: Color = Color::new([0, 0, 255, 255]);
pub const MAGENTA: Color = Color::new([255, 0, 255, 255]); // DEBUG/TEST ONLY
pub const CYAN: Color = Color::new([0, 255, 255, 255]);
pub const GREY: Color = GRAY;

pub fn clear(frame: &mut [u8]) {
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(BLACK.as_bytes());
    }
}
