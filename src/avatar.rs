// Discrete game avatars aka prefigured bags of components for game actors

use std::time;

use nalgebra_glm::Vec2;

use crate::{
    components::{
        CircleColliderCpt, ColorBodyCpt, DrawBodyCpt, DrawData, HumanInputCpt, MoveAttributesCpt,
        ProjectileEmitterCpt, RigidBodyCpt, RotatableBodyCpt, RotationalInputCpt, TransformCpt,
    },
    draw_bodies::generate_ship_lines,
    pixel::{BLUE, GREEN, WHITE, YELLOW},
    LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH,
};

pub struct Circloid;

impl Circloid {
    pub fn new() -> (TransformCpt, RigidBodyCpt, CircleColliderCpt, ColorBodyCpt) {
        (
            TransformCpt {
                position: Vec2::new(LOGICAL_WINDOW_WIDTH / 2., LOGICAL_WINDOW_HEIGHT / 2.),
                heading: 0.,
                scale: Vec2::new(1.0, 1.0),
            },
            RigidBodyCpt {
                velocity: Vec2::new(100., 100.),
            },
            CircleColliderCpt { r: 30.0 },
            ColorBodyCpt {
                primary: YELLOW,
                secondary: WHITE,
            },
        )
    }
}
pub struct Ship;

impl Ship {
    pub fn new() -> (
        TransformCpt,
        RigidBodyCpt,
        RotatableBodyCpt,
        MoveAttributesCpt,
        CircleColliderCpt,
        ColorBodyCpt,
        RotationalInputCpt,
        ProjectileEmitterCpt,
    ) {
        (
            TransformCpt::new(),
            RigidBodyCpt::new(),     // current velocity, used for physics
            RotatableBodyCpt::new(), // curent turn rate, used for physics
            MoveAttributesCpt::new(),
            CircleColliderCpt { r: 15.0 },
            ColorBodyCpt {
                primary: GREEN,
                secondary: BLUE,
            },
            RotationalInputCpt::new(),
            ProjectileEmitterCpt {
                projectile_velocity: Vec2::new(0., 0.),
                cooldown: 250,
                projectile_duration: time::Duration::new(0, 3000_000_000),
                hit_damage: 10,
                is_friendly: true,
                last_emission_time: None,
            },
        )
    }
}

pub struct HumanShip;
impl HumanShip {
    pub fn new() -> (
        TransformCpt,
        RigidBodyCpt,
        RotatableBodyCpt,
        MoveAttributesCpt,
        CircleColliderCpt,
        RotationalInputCpt,
        ProjectileEmitterCpt,
        DrawBodyCpt,
        HumanInputCpt,
    ) {
        (
            TransformCpt::new(),
            RigidBodyCpt::new(),     // current velocity, used for physics
            RotatableBodyCpt::new(), // curent turn rate, used for physics
            MoveAttributesCpt::new(),
            CircleColliderCpt { r: 15.0 },
            RotationalInputCpt::new(),
            ProjectileEmitterCpt::new(),
            DrawBodyCpt {
                colorbody: ColorBodyCpt {
                    primary: GREEN,
                    secondary: BLUE,
                },
                data: DrawData::Lines(generate_ship_lines()),
            },
            HumanInputCpt {},
        )
    }
}
