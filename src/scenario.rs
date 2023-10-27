// todo generate row of bouncing colored particles
// todo gen pair of pass-thru particles
// todo generate pair of pass-thru proj
// todo gen pair of non-colliding proj
// todo gen pair of non-coll and coll circloids
// todo gen pair of non-coll and coll ships (ensure heading flips accordingly)
// todo put it all in a scenario

use crate::{
    archetypes::{gen_particle, ArchParticle},
    pixel::*,
    LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH,
};

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

pub fn gen_passing_particles() -> Vec<ArchParticle> {
    let x_start = LOGICAL_WINDOW_WIDTH / 2.0 + 100.;
    vec![
        gen_particle(x_start + 5., 0f32, 0f32, -100., RED),
        gen_particle(x_start + 5., LOGICAL_WINDOW_HEIGHT, 0f32, 100., RED),
    ]
}
