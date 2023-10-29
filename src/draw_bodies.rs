use crate::{
    components::*, dev, draw::*, pixel::*, util::rotate_point, LOGICAL_WINDOW_HEIGHT,
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
pub fn draw_avatar(frame: &mut [u8], transform: &TransformCpt, drawbody: &DrawBodyCpt) {
    match drawbody {
        DrawBodyCpt { data, colorbody } => match data {
            DrawData::Lines(x) => {
                draw_body_of_lines(frame, transform, x.to_vec(), colorbody);
            }
            DrawData::R(r) => {
                draw_body_of_r(frame, transform, *r, colorbody);
            }
            DrawData::Particle => {
                draw_body_of_particle(frame, transform, colorbody);
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
                    transform.heading.get(),
                    transform.position.x,
                    transform.position.y,
                ),
                rotate_point(
                    t_pt2.x,
                    t_pt2.y,
                    transform.heading.get(),
                    transform.position.x,
                    transform.position.y,
                ),
            )
        })
        .collect()
}

pub fn draw_lines_by_list(
    frame: &mut [u8],
    vec: Vec<((f32, f32), (f32, f32))>,
    colorbody: &ColorBodyCpt,
) {
    for (vec1, vec2) in vec {
        draw_line(
            frame,
            vec1.0.round() as i32,
            vec1.1.round() as i32,
            vec2.0.round() as i32,
            vec2.1.round() as i32,
            colorbody.primary,
        );
    }
}

// DrawBodtCpt's DrawData::Lines
pub fn draw_body_of_lines(
    frame: &mut [u8],
    transform: &TransformCpt,
    lines: Vec<(Vec2, Vec2)>,
    colorbody: &ColorBodyCpt,
) {
    let transformed_line_endpoint_pairs = transform_body_data(transform, lines);
    draw_lines_by_list(frame, transformed_line_endpoint_pairs, colorbody);
}

// DrawBodtCpt's DrawData::R
pub fn draw_body_of_r(
    frame: &mut [u8],
    transform: &TransformCpt,
    r: f32,
    colorbody: &ColorBodyCpt,
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
    frame: &mut [u8],
    transform: &TransformCpt,
    r: f32,
    colorbody: &ColorBodyCpt,
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

pub fn draw_body_of_particle(frame: &mut [u8], transform: &TransformCpt, colorbody: &ColorBodyCpt) {
    let x = transform.position.x;
    let y = transform.position.y;
    draw_pixel(frame, x.round() as i32, y.round() as i32, colorbody.primary);
}

pub fn draw_boundary(frame: &mut [u8]) {
    let color = BLUE;
    let width = LOGICAL_WINDOW_WIDTH as i32 - 1;
    let height = LOGICAL_WINDOW_HEIGHT as i32 - 1;
    draw_line(frame, 0, 0, width, 0, color);
    draw_line(frame, width, 0, width, height, color);
    draw_line(frame, width, height, 0, height, color);
    draw_line(frame, 0, height, 0, 0, color);
}

pub fn draw_collision_rect(
    frame: &mut [u8],
    transform: &TransformCpt,
    collision_area: &BoxColliderCpt,
) {
    // ? cast or round then cast?
    draw_rect(
        frame,
        transform.position.x as i32,
        transform.position.y as i32,
        collision_area.w as i32,
        collision_area.h as i32,
        MAGENTA,
    );
}

pub fn draw_collision_circle(
    frame: &mut [u8],
    transform: &TransformCpt,
    collision_circle: &CircleColliderCpt,
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
