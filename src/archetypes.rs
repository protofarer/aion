use std::time;

use nalgebra_glm::Vec2;
use rand::Rng;

// avatar primitives
// - sets default component data
// - is a tuple of components, used for passing data around (eg avatar generation functions)
// - facilitates easy specification of avatars via generation functions
// - for specifying and exploring the "avatar design spaces"

use crate::{components::*, dev, pixel::*, LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH};

// ArchParticle
// - particle primitive
// - doesnt collide
pub type ArchParticle = (TransformCpt, RigidBodyCpt, DrawBodyCpt);

pub fn gen_particle(x: f32, y: f32, vx: f32, vy: f32, color: Color) -> ArchParticle {
    (
        TransformCpt {
            position: Vec2::new(x, y),
            heading: Theta::new(),
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

pub fn gen_particles(n: i32, x: f32, y: f32, vx: f32, vy: f32, color: Color) -> Vec<ArchParticle> {
    (0..n).map(|_| gen_particle(x, y, vx, vy, color)).collect()
}

pub fn gen_particle_rng() -> ArchParticle {
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
    (0..n).map(|_| gen_particle_rng()).collect()
}

// ArchCircloid
// - embodied circle, collidable
pub type ArchCircloid = (
    TransformCpt,
    RigidBodyCpt,
    DrawBodyCpt,
    CircleColliderCpt,
    HealthCpt,
);

pub fn gen_circloid(x: f32, y: f32, vx: f32, vy: f32, r: f32, color: Color) -> ArchCircloid {
    (
        TransformCpt {
            position: Vec2::new(x, y),
            heading: Theta::new(),
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
        HealthCpt::new(),
    )
}

pub fn gen_circloids(
    n: i32,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    r: f32,
    color: Color,
) -> Vec<ArchCircloid> {
    (0..n)
        .map(|_| gen_circloid(x, y, vx, vy, r, color))
        .collect()
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

// ArchProjectile
// - a collidable particle (for now aka ParticleProjectile, later impl AreaProjectile / CircloidProjectile / ...)
pub type ArchProjectile = (
    TransformCpt,
    RigidBodyCpt,
    DrawBodyCpt,
    ProjectileCpt,
    ParticleColliderCpt,
);

pub fn gen_projectile(
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    duration: time::Duration,
    hit_damage: i32,
    color: Color,
) -> ArchProjectile {
    (
        TransformCpt {
            position: Vec2::new(x, y),
            heading: Theta::new(),
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
        ProjectileCpt {
            is_friendly: false,
            hit_damage,
            duration,
            start_time: time::Instant::now(),
        },
        ParticleColliderCpt {},
    )
}
pub fn gen_projectiles(
    n: i32,
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    duration: time::Duration,
    color: Color,
) -> Vec<ArchProjectile> {
    (0..n)
        .map(|_| gen_projectile(x, y, vx, vy, duration, 10, color))
        .collect()
}

pub fn gen_projectile_rng_all() -> ArchProjectile {
    let mut rng = rand::thread_rng();
    let sixtyfour = rng.gen::<u64>();
    dev!("rng gen u64: {}", sixtyfour);
    let mut rng_int = rng.gen::<i32>();
    let mut sign = (rng_int / rng_int.abs()) as f32;
    gen_projectile(
        rng.gen::<f32>() * LOGICAL_WINDOW_WIDTH,
        rng.gen::<f32>() * LOGICAL_WINDOW_HEIGHT,
        rng.gen::<f32>() * 1000.0 * sign,
        rng.gen::<f32>() * 1000.0 * sign,
        time::Duration::new(10 * sixtyfour, 0),
        10,
        Color::rng(),
    )
}

pub fn gen_buncha_rng_projectiles(n: i32) -> Vec<ArchProjectile> {
    (0..n).map(|_| gen_projectile_rng_all()).collect()
}
