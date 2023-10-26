use crate::{
    components::*, dev, draw::*, geom::rotate_point, pixel::*, LOGICAL_WINDOW_HEIGHT,
    LOGICAL_WINDOW_WIDTH,
};
use nalgebra_glm::Vec2;

// Body draw functions are an interface between floating point data coming in
// from the components and running update() on them. They add a layer of
// separation from the raw component data and the primitive drawing functions.
// The body functions know about various data pertaining to avatars (archetypes
// + game avatar form and function) and, in particular, do the rounding and
// casting from floats (flexible and costly, world data and higher level
// functions) to integers (discrete and performant, primitive draw)

// decides which body to draw
pub fn draw_avatar(transform: &TransformCpt, drawbody: &DrawBodyCpt, frame: &mut [u8]) {
    match drawbody {
        DrawBodyCpt { data, colorbody } => match data {
            DrawData::Lines(x) => {
                draw_body_of_lines(transform, x.to_vec(), colorbody, frame);
            }
            DrawData::R(r) => {
                draw_body_of_r(transform, *r, colorbody, frame);
            }
            DrawData::Particle => {
                draw_body_of_particle(transform, colorbody, frame);
            }
            _ => {}
        },
        _ => {}
    }
}

fn transform_body_data(
    transform: &TransformCpt,
    lines: Vec<(Vec2, Vec2)>,
) -> Vec<((f32, f32), (f32, f32))> {
    // transform relative to center and heading
    lines
        .iter()
        .map(|(pt1, pt2)| {
            (
                Vec2::new(pt1.x + transform.position.x, pt1.y + transform.position.y),
                Vec2::new(pt2.x + transform.position.x, pt2.y + transform.position.y),
            )
        })
        .map(|(t_pt1, t_pt2)| {
            (
                rotate_point(
                    t_pt1.x,
                    t_pt1.y,
                    transform.heading,
                    transform.position.x,
                    transform.position.y,
                ),
                rotate_point(
                    t_pt2.x,
                    t_pt2.y,
                    transform.heading,
                    transform.position.x,
                    transform.position.y,
                ),
            )
        })
        .collect()
}

pub fn draw_lines_by_list(
    vec: Vec<((f32, f32), (f32, f32))>,
    colorbody: &ColorBodyCpt,
    frame: &mut [u8],
) {
    for (vec1, vec2) in vec {
        draw_line(
            vec1.0.round() as i32,
            vec1.1.round() as i32,
            vec2.0.round() as i32,
            vec2.1.round() as i32,
            colorbody.primary,
            frame,
        );
    }
}

// DrawBodtCpt's DrawData::Lines
pub fn draw_body_of_lines(
    transform: &TransformCpt,
    lines: Vec<(Vec2, Vec2)>,
    colorbody: &ColorBodyCpt,
    frame: &mut [u8],
) {
    let transformed_line_endpoint_pairs = transform_body_data(transform, lines);
    draw_lines_by_list(transformed_line_endpoint_pairs, colorbody, frame);
}

// DrawBodtCpt's DrawData::R
pub fn draw_body_of_r(
    transform: &TransformCpt,
    r: f32,
    colorbody: &ColorBodyCpt,
    frame: &mut [u8],
) {
    draw_circle(
        frame,
        (transform.position.x) as i32,
        (transform.position.y) as i32,
        r as i32,
        colorbody.primary,
    );
}

pub fn draw_body_of_circle(
    transform: &TransformCpt,
    r: f32,
    colorbody: &ColorBodyCpt,
    frame: &mut [u8],
) {
}

pub fn generate_ship_lines() -> Vec<(Vec2, Vec2)> {
    // relative to center (circle collisions and rotational controls)
    const SHIP_R: f32 = 15.;

    let r = SHIP_R;
    let rad2 = nalgebra_glm::root_two::<f32>();

    let dy = SHIP_R / rad2;
    let dx = SHIP_R / rad2;

    let pts = [(-dx, -dy), (-0.5 * r, 0.), (-dx, dy), (r, 0.), (-dx, -dy)];

    let mut v = vec![];
    for i in 0..(pts.len() - 1) {
        let p1 = Vec2::new(pts[i].0, pts[i].1);
        let p2 = Vec2::new(pts[i + 1].0, pts[i + 1].1);
        v.push((p1, p2));
    }
    v
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

pub fn draw_body_of_particle(transform: &TransformCpt, colorbody: &ColorBodyCpt, frame: &mut [u8]) {
    let x = transform.position.x;
    let y = transform.position.y;
    draw_pixel(x.round() as i32, y.round() as i32, colorbody.primary, frame);
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

pub fn draw_collision_rect(
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

// pub fn draw_box(transform: &TransformCpt, colorbody: &ColorBodyCpt, frame: &mut [u8]) {
//     let r = 15.;
//     let x = transform.position.x;
//     let y = transform.position.y;

//     let mut x1 = x;
//     let mut y1 = y;

//     let mut x2 = x1 + r;
//     let mut y2 = y1;

//     let mut x3 = x2;
//     let mut y3 = y2 + r;

//     let mut x4 = x1;
//     let mut y4 = y3;

//     let cx = x1 + (r / 2.0);
//     let cy = y1 + (r / 2.0);

//     (x1, y1) = rotate_point(x1, y1, transform.heading, cx, cy);
//     (x2, y2) = rotate_point(x2, y2, transform.heading, cx, cy);
//     (x3, y3) = rotate_point(x3, y3, transform.heading, cx, cy);
//     (x4, y4) = rotate_point(x4, y4, transform.heading, cx, cy);

//     // Draw the triangle
//     draw_line(
//         x1.round() as i32,
//         y1.round() as i32,
//         x2.round() as i32,
//         y2.round() as i32,
//         colorbody.primary,
//         frame,
//     );

//     draw_line(
//         x2.round() as i32,
//         y2.round() as i32,
//         x3.round() as i32,
//         y3.round() as i32,
//         colorbody.primary,
//         frame,
//     );
//     draw_line(
//         x3.round() as i32,
//         y3.round() as i32,
//         x4.round() as i32,
//         y4.round() as i32,
//         colorbody.primary,
//         frame,
//     );
//     draw_line(
//         x4.round() as i32,
//         y4.round() as i32,
//         x1.round() as i32,
//         y1.round() as i32,
//         colorbody.primary,
//         frame,
//     );
// }
