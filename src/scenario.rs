use crate::{archetypes::*, pixel::*, LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH};
use hecs::World;
use std::time;

pub fn gen_small_circloid(x: f32, y: f32, vx: f32, vy: f32, color: Color) -> ArchCircloid {
    gen_circloid(x, y, vx, vy, 10., color)
}

// ~35 px wide
pub fn gen_row_particles(x_start: f32) -> Vec<ArchParticle> {
    (0..8)
        .map(|i| {
            let color = match i {
                0 => WHITE,
                1 => RED,
                2 => ORANGE,
                3 => YELLOW,
                4 => GREEN,
                5 => BLUE,
                6 => CYAN,
                _ => GRAY,
            };
            let x = x_start + (i as f32 * 5.);
            gen_particle(x, 0., 0., -100., color)
        })
        .collect()
}

// ~5 px wide
pub fn gen_intersecting_particles(x_start: f32) -> Vec<ArchParticle> {
    vec![
        gen_particle(x_start, 0f32, 0f32, -100., WHITE),
        gen_particle(x_start, LOGICAL_WINDOW_HEIGHT, 0f32, 100., WHITE),
    ]
}

// ~ 5 px wide
pub fn gen_intersecting_projectiles(x_start: f32) -> Vec<ArchProjectile> {
    vec![
        gen_projectile(
            x_start,
            0f32,
            0f32,
            -100.,
            time::Duration::new(100, 0),
            10,
            RED,
        ),
        gen_projectile(
            x_start,
            LOGICAL_WINDOW_HEIGHT,
            0f32,
            100.,
            time::Duration::new(100, 0),
            10,
            RED,
        ),
    ]
}

// ~30 px wide
pub fn gen_colliding_circloids(x_start: f32) -> Vec<ArchCircloid> {
    vec![
        gen_small_circloid(x_start, 10., 0f32, -100., GREEN),
        gen_small_circloid(x_start, LOGICAL_WINDOW_HEIGHT - 10., 0f32, -100., BLUE),
    ]
}

// ~30px wide
pub fn gen_intersecting_circloid_projectile(x_start: f32) -> (ArchProjectile, ArchCircloid) {
    (
        gen_projectile(x_start, 0f32, 0., 100., time::Duration::new(10, 0), 10, RED),
        gen_small_circloid(x_start, LOGICAL_WINDOW_HEIGHT - 10., 0f32, -100., YELLOW),
    )
}

pub fn spawn_scenario1(world: &mut World) {
    let x_start = LOGICAL_WINDOW_WIDTH / 2.0;
    world.spawn_batch(gen_row_particles(x_start));
    world.spawn(gen_small_circloid(
        x_start + 30.,
        LOGICAL_WINDOW_HEIGHT - 10.,
        0.,
        -100.,
        ORANGE,
    )); // intersect with particles
    world.spawn_batch(gen_intersecting_particles(x_start + 50.));
    world.spawn_batch(gen_intersecting_projectiles(x_start + 55.));
    world.spawn_batch(gen_colliding_circloids(x_start + 75.));
    let (proj, circ) = gen_intersecting_circloid_projectile(x_start + 100.);
    world.spawn(proj);
    world.spawn(circ);
}
