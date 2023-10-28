// todo generate row of bouncing colored particles
// todo gen pair of pass-thru particles
// todo gen a non-colliding proj
// todo generate pair of pass-thru proj
// todo gen pair of non-coll and coll circloids
// todo gen pair of non-coll and coll ships (ensure heading flips accordingly)
// todo put it all in a scenario

use std::time;

use hecs::World;

use crate::{archetypes::*, pixel::*, LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH};

pub fn gen_row_particles() -> Vec<ArchParticle> {
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
            let x = (LOGICAL_WINDOW_WIDTH / 2.0) + (i as f32 * 10.);
            gen_particle(x, 0., 0., -100., color)
        })
        .collect()
}

pub fn gen_intersecting_circloid_particles() -> ArchCircloid {
    let x = (LOGICAL_WINDOW_WIDTH / 2.0);
    gen_circloid(x, LOGICAL_WINDOW_HEIGHT - 0.15, 0., -100., 15., WHITE)
}

pub fn gen_intersecting_particles() -> Vec<ArchParticle> {
    let x_start = LOGICAL_WINDOW_WIDTH / 2.0 + 80.;
    vec![
        gen_particle(x_start + 5., 0f32, 0f32, -100., RED),
        gen_particle(x_start + 5., LOGICAL_WINDOW_HEIGHT, 0f32, 100., RED),
    ]
}

pub fn gen_intersecting_projectiles() -> Vec<ArchProjectile> {
    let x_start = LOGICAL_WINDOW_WIDTH / 2.0 + 100.;
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
            GREEN,
        ),
    ]
}

pub fn gen_colliding_circloids() -> Vec<ArchCircloid> {
    let x_start = LOGICAL_WINDOW_WIDTH / 2.0 + 140.;
    vec![
        gen_circloid(x_start, 15f32, 0f32, -100., 15., GREEN),
        gen_circloid(x_start, LOGICAL_WINDOW_HEIGHT - 15., 0f32, 100., 15., BLUE),
    ]
}

pub fn gen_intersecting_circloid_projectile() -> (ArchProjectile, ArchCircloid) {
    let x_start = LOGICAL_WINDOW_WIDTH / 2.0 + 180.;
    (
        gen_projectile(x_start, 0f32, 0., 100., time::Duration::new(5, 0), 10, RED),
        gen_circloid(
            x_start,
            LOGICAL_WINDOW_HEIGHT - 20.,
            0f32,
            -100.,
            15.,
            GREEN,
        ),
    )
}

pub fn spawn_scenario1(world: &mut World) {
    world.spawn_batch(gen_row_particles());
    world.spawn(gen_intersecting_circloid_particles());
    world.spawn_batch(gen_intersecting_particles());
    world.spawn_batch(gen_intersecting_projectiles());
    world.spawn_batch(gen_colliding_circloids());
    let (proj, circ) = gen_intersecting_circloid_projectile();
    world.spawn(proj);
    world.spawn(circ);
}
