use crate::pixel::{Color, BLUE, CYAN, GREEN, GREY, MAGENTA, ORANGE, RED, WHITE, YELLOW};
use hecs::Entity;
use nalgebra_glm::Vec2;
use std::{default, time};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct TransformCpt {
    pub position: Vec2,
    pub heading: f32,
    pub scale: Vec2,
}
impl TransformCpt {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(100., 100.),
            heading: 0.,
            scale: Vec2::new(1., 1.),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct RigidBodyCpt {
    pub velocity: Vec2,
}
impl RigidBodyCpt {
    pub fn new() -> Self {
        Self {
            velocity: Vec2::new(1., 0.),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct RotatableBodyCpt {
    pub rotation_rate: f32,
}
impl RotatableBodyCpt {
    pub fn new() -> Self {
        Self { rotation_rate: 1.0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Turn {
    #[default]
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct RotationalInputCpt {
    pub turn_sign: Option<Turn>,
    pub is_thrusting: bool,
}
impl RotationalInputCpt {
    pub fn new() -> Self {
        Self {
            turn_sign: None,
            is_thrusting: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct MoveAttributesCpt {
    pub speed: f32,
    pub turn_rate: f32,
}
impl MoveAttributesCpt {
    pub fn new() -> Self {
        Self {
            speed: 500.0,
            turn_rate: 12.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct HumanInputCpt {}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct BoxColliderCpt {
    pub w: f32,
    pub h: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct CircleColliderCpt {
    pub r: f32,
}
impl CircleColliderCpt {
    pub fn new() -> Self {
        Self { r: 10.0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ParticleColliderCpt {}

impl ParticleColliderCpt {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct ColorBodyCpt {
    pub primary: Color,
    pub secondary: Color,
}
impl ColorBodyCpt {
    pub fn new() -> Self {
        Self {
            primary: WHITE,
            secondary: BLUE,
        }
    }
    pub fn red() -> Self {
        Self {
            primary: RED,
            secondary: WHITE,
        }
    }
    pub fn white() -> Self {
        Self {
            primary: WHITE,
            secondary: RED,
        }
    }
    pub fn green() -> Self {
        Self {
            primary: GREEN,
            secondary: WHITE,
        }
    }
    pub fn blue() -> Self {
        Self {
            primary: BLUE,
            secondary: WHITE,
        }
    }
    pub fn orange() -> Self {
        Self {
            primary: ORANGE,
            secondary: WHITE,
        }
    }
    pub fn yellow() -> Self {
        Self {
            primary: YELLOW,
            secondary: WHITE,
        }
    }
    pub fn magenta() -> Self {
        Self {
            primary: MAGENTA,
            secondary: WHITE,
        }
    }
    pub fn cyan() -> Self {
        Self {
            primary: CYAN,
            secondary: WHITE,
        }
    }
    pub fn grey() -> Self {
        Self {
            primary: GREY,
            secondary: WHITE,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct TranslationalInputCpt {
    pub direction: Option<Direction>,
}
impl TranslationalInputCpt {
    pub fn new() -> Self {
        Self { direction: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct CraftActionStateCpt {
    pub is_firing_primary: bool,
    pub is_firing_secondary: bool,
}
impl CraftActionStateCpt {
    pub fn new() -> Self {
        Self {
            is_firing_primary: false,
            is_firing_secondary: false,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct MindStateCpt {
    pub is_sensing: bool,
    pub is_using_primary: bool,
}
impl MindStateCpt {
    pub fn new() -> Self {
        Self {
            is_sensing: false,
            is_using_primary: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectileEmitterCpt {
    pub projectile_speed: f32,
    pub cooldown: i32,
    pub projectile_duration: time::Duration,
    pub hit_damage: i32,
    pub is_friendly: bool,
    pub last_emission_time: time::Instant,
    pub intends_to_fire: bool,
}

impl ProjectileEmitterCpt {
    pub fn new() -> Self {
        Self {
            projectile_speed: 300.,
            cooldown: 250,
            projectile_duration: time::Duration::new(7, 0),
            hit_damage: 10,
            is_friendly: false,
            last_emission_time: time::Instant::now(),
            intends_to_fire: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectileCpt {
    pub is_friendly: bool,
    pub hit_damage: i32,
    pub duration: time::Duration,
    pub start_time: time::Instant,
}
impl ProjectileCpt {
    pub fn new() -> Self {
        Self {
            is_friendly: false,
            hit_damage: 0,
            duration: time::Duration::new(0, 3_000_000_000),
            start_time: time::Instant::now(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum Direction {
    N,
    NE,
    #[default]
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DrawData {
    R(f32),
    Lines(Vec<(Vec2, Vec2)>),
    Particle,
}

impl DrawData {
    pub fn new() -> DrawData {
        DrawData::default_circle()
    }
    pub fn default_box() -> DrawData {
        DrawData::Lines(vec![
            (Vec2::new(-5., -5.), Vec2::new(5., -5.)),
            (Vec2::new(5., -5.), Vec2::new(5., 5.)),
            (Vec2::new(5., 5.), Vec2::new(-5., 5.)),
            (Vec2::new(-5., 5.), Vec2::new(-5., -5.)),
        ])
    }
    pub fn default_circle() -> DrawData {
        DrawData::R(10.)
    }
    pub fn default_partile() -> DrawData {
        DrawData::Particle
    }
}

// cant use copy because DrawData has a Vec type, must clone where needed
#[derive(Clone, Debug, PartialEq)]
pub struct DrawBodyCpt {
    pub data: DrawData,
    pub colorbody: ColorBodyCpt,
}

impl DrawBodyCpt {
    pub fn new() -> DrawBodyCpt {
        DrawBodyCpt {
            colorbody: ColorBodyCpt::new(),
            data: DrawData::new(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CollisionDetectionEvent {
    pub a: Entity,
    pub b: Entity,
}
