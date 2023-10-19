use crate::{components::ColorBody, pixel::Color};

use super::{LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH};

pub fn draw_pixel(x: i32, y: i32, color: Color, frame: &mut [u8]) {
    if x >= 0 && x < (4 * LOGICAL_WINDOW_WIDTH as i32) && y >= 0 && y < LOGICAL_WINDOW_HEIGHT as i32
    {
        let i = ((LOGICAL_WINDOW_WIDTH as i32 * y + x) * 4) as usize;
        frame[i..i + 4].copy_from_slice(color.as_bytes());
    }
}

pub fn draw_rect(x: i32, y: i32, width: i32, height: i32, color: Color, frame: &mut [u8]) {
    for i in y..=y + height {
        for j in x..=x + width {
            let n = (LOGICAL_WINDOW_WIDTH as i32 * i + j) as usize;
            if (i == y || i == y + height) && (j >= x && j <= x + width) {
                frame[n..n + 4].copy_from_slice(color.as_bytes());
            }
            if (j == x || j == x + width) && (i >= y && i <= y + height) {
                frame[n..n + 4].copy_from_slice(color.as_bytes());
            }
        }
    }
}

pub fn draw_line(x0: i32, y0: i32, x1: i32, y1: i32, color: Color, frame: &mut [u8]) {
    let x_len = x1 - x0;
    let y_len = y1 - y0;

    let longer_side_len = if x_len.abs() >= y_len.abs() {
        x_len.abs()
    } else {
        y_len.abs()
    };

    let dx = x_len as f32 / longer_side_len as f32;
    let dy = y_len as f32 / longer_side_len as f32;

    let mut x = x0 as f32;
    let mut y = y0 as f32;

    for i in 0..=longer_side_len as i32 {
        draw_pixel(x.round() as i32, y.round() as i32, color, frame);
        x += dx;
        y += dy;
    }
}
