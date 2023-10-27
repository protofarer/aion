use nalgebra_glm::Vec2;
use rand::Rng;

// avatar primitives
// - sets default component data
// - is a tuple of components, used for passing data around (eg avatar generation functions)
// - facilitates easy specification of avatars via generation functions
// - for specifying and exploring the "avatar design spaces"

use crate::{components::*, pixel::*, LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH};

pub type ArchParticle = (TransformCpt, RigidBodyCpt, DrawBodyCpt);

pub fn gen_particle(x: f32, y: f32, vx: f32, vy: f32, color: Color) -> ArchParticle {
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
    )
}

pub fn gen_particle_rng_all() -> ArchParticle {
    let mut rng = rand::thread_rng();
    let mut rng_int = rng.gen::<i32>();
    let mut sign = (rng_int / rng_int.abs()) as f32;
    gen_particle(
        rng.gen::<f32>() * LOGICAL_WINDOW_WIDTH,
        rng.gen::<f32>() * LOGICAL_WINDOW_HEIGHT,
        rng.gen::<f32>() * 1000.0 * sign,
        rng.gen::<f32>() * 1000.0 * sign,
        Color::rng(),
    )
}

pub fn gen_buncha_rng_particles(n: i32) -> Vec<ArchParticle> {
    (0..n).map(|_| gen_particle_rng_all()).collect()
}

pub type ArchCircloid = (TransformCpt, RigidBodyCpt, DrawBodyCpt, CircleColliderCpt);

pub fn gen_circloid(x: f32, y: f32, vx: f32, vy: f32, r: f32, color: Color) -> ArchCircloid {
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
                primary: Color::RGB(160, 160, 0),
                secondary: Color::RGB(0, 0, 0),
            },
            data: DrawData::R(r),
        },
        CircleColliderCpt { r },
    )
}

pub fn gen_circloid_rng() -> ArchCircloid {
    let mut rng = rand::thread_rng();
    let r = (rng.gen::<f32>() * 40.) + 10.;
    let mut rng_int = rng.gen::<i32>();
    let mut sign = (rng_int / rng_int.abs()) as f32;
    gen_circloid(
        rng.gen::<f32>() * (LOGICAL_WINDOW_WIDTH - r),
        rng.gen::<f32>() * (LOGICAL_WINDOW_HEIGHT - r),
        (rng.gen::<f32>() * 300.0 + 100.) * sign,
        (rng.gen::<f32>() * 300.0 + 100.) * sign,
        r,
        Color::rng(),
    )
}

pub fn gen_buncha_rng_circloids(n: i32) -> Vec<ArchCircloid> {
    (0..n).map(|_| gen_circloid_rng()).collect()
}

pub fn gen_circloids(n: i32) -> Vec<ArchCircloid> {
    let mut circles = vec![];
    for i in 0..n {
        circles.push(gen_circloid_rng());
    }
    circles
}

type ArchProjectile = (TransformCpt, RigidBodyCpt, DrawBodyCpt, ParticleColliderCpt);

pub fn gen_projectile(x: f32, y: f32, vx: f32, vy: f32, color: Color) -> ArchProjectile {
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

pub fn gen_projectile_rng_all() -> ArchProjectile {
    let mut rng = rand::thread_rng();
    let mut rng_int = rng.gen::<i32>();
    let mut sign = (rng_int / rng_int.abs()) as f32;
    gen_projectile(
        rng.gen::<f32>() * LOGICAL_WINDOW_WIDTH,
        rng.gen::<f32>() * LOGICAL_WINDOW_HEIGHT,
        rng.gen::<f32>() * 1000.0 * sign,
        rng.gen::<f32>() * 1000.0 * sign,
        Color::rng(),
    )
}

pub fn gen_buncha_rng_projectiles(n: i32) -> Vec<ArchProjectile> {
    (0..n).map(|_| gen_projectile_rng_all()).collect()
}
