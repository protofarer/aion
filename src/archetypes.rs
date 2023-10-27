use nalgebra_glm::Vec2;
use rand::Rng;

// avatar primitives
// good for exploring design, testing, developing extensible system for defining avatars,
// that is, exploring the "avatar design space"
use crate::{components::*, pixel::*, LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH};

pub struct ArchParticle(TransformCpt, RigidBodyCpt, DrawBodyCpt);
impl ArchParticle {
    pub fn into_tuple(self) -> (TransformCpt, RigidBodyCpt, DrawBodyCpt) {
        let ArchParticle(a, b, c) = self;
        (a, b, c)
    }
}

pub fn gen_particle(x: f32, y: f32, vx: f32, vy: f32, color: Color) -> ArchParticle {
    ArchParticle(
        TransformCpt {
            position: Vec2::new(x, y),
            heading: 0.0,
            scale: Vec2::new(1.0, 1.0),
        },
        RigidBodyCpt {
            velocity: Vec2::new(vx, vy),
        },
        DrawBodyCpt {
            colorbody: ColorBodyCpt {
                primary: color,
                secondary: WHITE,
            },
            data: DrawData::Particle,
        },
    )
}

pub fn gen_buncha_rng_particles(n: i32) -> Vec<ArchParticle> {
    (0..n).map(|_| gen_particle_rng_all()).collect()
}

pub fn gen_particle_rng_all() -> ArchParticle {
    let mut rng = rand::thread_rng();
    gen_particle(
        rng.gen::<f32>() * LOGICAL_WINDOW_WIDTH,
        rng.gen::<f32>() * LOGICAL_WINDOW_HEIGHT,
        rng.gen::<f32>() * 1000.0,
        rng.gen::<f32>() * 1000.0,
        Color::rng(),
    )
}

pub fn gen_projectile(
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
) -> (TransformCpt, RigidBodyCpt, DrawBodyCpt, ParticleColliderCpt) {
    (
        TransformCpt {
            position: Vec2::new(x, y),
            heading: 0.0,
            scale: Vec2::new(1.0, 1.0),
        },
        RigidBodyCpt {
            velocity: Vec2::new(vx, vy),
        },
        DrawBodyCpt {
            colorbody: ColorBodyCpt {
                primary: color,
                secondary: WHITE,
            },
            data: DrawData::Particle,
        },
        ParticleColliderCpt {},
    )
}

pub fn gen_circloid() -> (TransformCpt, RigidBodyCpt, CircleColliderCpt, ColorBodyCpt) {
    let mut rng = rand::thread_rng();
    (
        TransformCpt {
            position: Vec2::new(
                rng.gen::<f32>() * LOGICAL_WINDOW_WIDTH,
                rng.gen::<f32>() * LOGICAL_WINDOW_HEIGHT,
            ),
            heading: 0.0,
            scale: Vec2::new(1.0, 1.0),
        },
        RigidBodyCpt {
            velocity: Vec2::new(rng.gen::<f32>() * 100.0, rng.gen::<f32>() * 100.0),
        },
        CircleColliderCpt { r: 30.0 },
        ColorBodyCpt {
            primary: Color::RGB(160, 160, 0),
            secondary: Color::RGB(0, 0, 0),
        },
    )
}

pub fn gen_circloids(n: i32) -> Vec<(TransformCpt, RigidBodyCpt, CircleColliderCpt, ColorBodyCpt)> {
    let mut circles = vec![];
    for i in 0..n {
        circles.push(gen_circloid());
    }
    circles
}
