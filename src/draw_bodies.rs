use crate::{
    components::*, draw::*, geom::rotate_point, pixel::*, LOGICAL_WINDOW_HEIGHT,
    LOGICAL_WINDOW_WIDTH,
};

pub fn draw_ship(transform: &Transform, colorbody: &ColorBody, frame: &mut [u8]) {
    let r = 15.0;

    let x = transform.position.x;
    let y = transform.position.y;

    let mut x1 = x - r / 2.0;
    let mut y1 = y - r / 2.0;

    let mut x2 = x1;
    let mut y2 = y + r / 2.0;

    let mut x3 = x + r;
    let mut y3 = y;

    let mut xm = x + r / 20.0;
    let mut ym = y;

    let cx = (x1 + x2 + x3) / 3.0;
    let cy = (y1 + y2 + y3) / 3.0;

    (x1, y1) = rotate_point(x1, y1, transform.rotation, cx, cy);
    (xm, ym) = rotate_point(xm, ym, transform.rotation, cx, cy);
    (x2, y2) = rotate_point(x2, y2, transform.rotation, cx, cy);
    (x3, y3) = rotate_point(x3, y3, transform.rotation, cx, cy);

    // Draw the triangle
    draw_line(
        x1.round() as i32,
        y1.round() as i32,
        xm.round() as i32,
        ym.round() as i32,
        colorbody.primary,
        frame,
    );

    draw_line(
        xm.round() as i32,
        ym.round() as i32,
        x2.round() as i32,
        y2.round() as i32,
        colorbody.primary,
        frame,
    );
    draw_line(
        x2.round() as i32,
        y2.round() as i32,
        x3.round() as i32,
        y3.round() as i32,
        colorbody.primary,
        frame,
    );
    draw_line(
        x3.round() as i32,
        y3.round() as i32,
        x1.round() as i32,
        y1.round() as i32,
        colorbody.primary,
        frame,
    );
}
pub fn draw_box(transform: &Transform, colorbody: &ColorBody, frame: &mut [u8]) {
    let r = 15.0;

    let x = transform.position.x;
    let y = transform.position.y;

    let mut x1 = x;
    let mut y1 = y;

    let mut x2 = x1 + r;
    let mut y2 = y1;

    let mut x3 = x2;
    let mut y3 = y2 + r;

    let mut x4 = x1;
    let mut y4 = y3;

    let cx = x1 + (r / 2.0);
    let cy = y1 + (r / 2.0);

    (x1, y1) = rotate_point(x1, y1, transform.rotation, cx, cy);
    (x2, y2) = rotate_point(x2, y2, transform.rotation, cx, cy);
    (x3, y3) = rotate_point(x3, y3, transform.rotation, cx, cy);
    (x4, y4) = rotate_point(x4, y4, transform.rotation, cx, cy);

    // Draw the triangle
    draw_line(
        x1.round() as i32,
        y1.round() as i32,
        x2.round() as i32,
        y2.round() as i32,
        colorbody.primary,
        frame,
    );

    draw_line(
        x2.round() as i32,
        y2.round() as i32,
        x3.round() as i32,
        y3.round() as i32,
        colorbody.primary,
        frame,
    );
    draw_line(
        x3.round() as i32,
        y3.round() as i32,
        x4.round() as i32,
        y4.round() as i32,
        colorbody.primary,
        frame,
    );
    draw_line(
        x4.round() as i32,
        y4.round() as i32,
        x1.round() as i32,
        y1.round() as i32,
        colorbody.primary,
        frame,
    );
}

pub fn draw_particle(transform: &Transform, colorbody: &ColorBody, frame: &mut [u8]) {
    // let r = 15.0;

    let x = transform.position.x;
    let y = transform.position.y;

    // let mut x1 = x;
    // let mut y1 = y;

    // let mut x2 = x1 + r;
    // let mut y2 = y1;

    // let mut x3 = x2;
    // let mut y3 = y2 + r;

    // let mut x4 = x1;
    // let mut y4 = y3;

    // let cx = x1 + (r / 2.0);
    // let cy = y1 + (r / 2.0);

    // (x1, y1) = rotate_point(x1, y1, transform.rotation, cx, cy);
    // (x2, y2) = rotate_point(x2, y2, transform.rotation, cx, cy);
    // (x3, y3) = rotate_point(x3, y3, transform.rotation, cx, cy);
    // (x4, y4) = rotate_point(x4, y4, transform.rotation, cx, cy);

    // Draw the triangle
    draw_pixel(x as i32, y as i32, colorbody.primary, frame);
}

pub fn draw_boundary(frame: &mut [u8]) {
    let color = BLUE;
    let width = LOGICAL_WINDOW_WIDTH as i32 - 1;
    let height = LOGICAL_WINDOW_HEIGHT as i32 - 1;
    draw_line(0, 0, width, 0, color, frame);
    draw_line(width, 0, width, height, color, frame);
    draw_line(width, height, 0, height, color, frame);
    draw_line(0, height, 0, 0, color, frame);
}
