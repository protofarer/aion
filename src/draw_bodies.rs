use crate::{
    components::*, draw::*, geom::rotate_point, pixel::*, LOGICAL_WINDOW_HEIGHT,
    LOGICAL_WINDOW_WIDTH,
};

pub fn draw_ship_circle_collision(
    transform: &TransformCpt,
    cc: &CircleColliderCpt,
    colorbody: &ColorBodyCpt,
    frame: &mut [u8],
) {
    // r is canonical draw length, half a side of collision square
    // or the length of radius for collision circle
    let r = cc.r;
    let dy = r * (nalgebra_glm::pi::<f32>() / 4.0).sin();
    let dx = r * (nalgebra_glm::pi::<f32>() / 4.0).cos();

    let xc = transform.position.x;
    let yc = transform.position.y;

    let mut x1 = xc - dx;
    let mut y1 = yc - dy;

    // notch
    let mut xm = xc - 0.5 * r;
    let mut ym = yc;

    let mut x2 = xc - dx;
    let mut y2 = yc + dy;

    let mut x3 = xc + r;
    let mut y3 = yc;

    (x1, y1) = rotate_point(x1, y1, transform.heading, xc, yc);
    (xm, ym) = rotate_point(xm, ym, transform.heading, xc, yc);
    (x2, y2) = rotate_point(x2, y2, transform.heading, xc, yc);
    (x3, y3) = rotate_point(x3, y3, transform.heading, xc, yc);

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

pub fn draw_ship_square(
    transform: &TransformCpt,
    bc: &BoxColliderCpt,
    colorbody: &ColorBodyCpt,
    frame: &mut [u8],
) {
    // r is canonical draw length, half a side of collision square
    // or the length of radius for collision circle
    let r = bc.w / 2.0;

    let x = transform.position.x;
    let y = transform.position.y;

    let mut x1 = x;
    let mut y1 = y;

    // notch
    let mut xm = x + r * 0.5;
    let mut ym = y + r;

    let mut x2 = x1;
    let mut y2 = y + r * 2.0;

    let mut x3 = x + r * 2.0;
    let mut y3 = y + r;

    let cx = x + r;
    let cy = y + r;

    (x1, y1) = rotate_point(x1, y1, transform.heading, cx, cy);
    (xm, ym) = rotate_point(xm, ym, transform.heading, cx, cy);
    (x2, y2) = rotate_point(x2, y2, transform.heading, cx, cy);
    (x3, y3) = rotate_point(x3, y3, transform.heading, cx, cy);

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
pub fn draw_box(transform: &TransformCpt, colorbody: &ColorBodyCpt, frame: &mut [u8]) {
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

    (x1, y1) = rotate_point(x1, y1, transform.heading, cx, cy);
    (x2, y2) = rotate_point(x2, y2, transform.heading, cx, cy);
    (x3, y3) = rotate_point(x3, y3, transform.heading, cx, cy);
    (x4, y4) = rotate_point(x4, y4, transform.heading, cx, cy);

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

pub fn draw_particle(transform: &TransformCpt, colorbody: &ColorBodyCpt, frame: &mut [u8]) {
    let x = transform.position.x;
    let y = transform.position.y;
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

pub fn draw_circloid(
    transform: &TransformCpt,
    collision_area: &BoxColliderCpt,
    colorbody: &ColorBodyCpt,
    frame: &mut [u8],
) {
    let r = collision_area.w / 2.0;
    draw_circle(
        frame,
        (transform.position.x + r) as i32,
        (transform.position.y + r) as i32,
        r as i32,
        colorbody.primary,
    );
}

pub fn draw_collision_square(
    transform: &TransformCpt,
    collision_area: &BoxColliderCpt,
    frame: &mut [u8],
) {
    // ? cast or round then cast?
    draw_rect(
        transform.position.x as i32,
        transform.position.y as i32,
        collision_area.w as i32,
        collision_area.h as i32,
        MAGENTA,
        frame,
    );
}

pub fn draw_collision_circle(
    transform: &TransformCpt,
    collision_circle: &CircleColliderCpt,
    frame: &mut [u8],
) {
    // ? cast or round then cast?
    draw_circle(
        frame,
        transform.position.x as i32,
        transform.position.y as i32,
        collision_circle.r as i32,
        MAGENTA,
    );
}
