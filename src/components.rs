use crate::pixel::{Color, BLUE, WHITE};
use nalgebra_glm::Vec2;
use std::time;

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RotatableBodyCpt {
    pub rotation_rate: f32,
}
impl RotatableBodyCpt {
    pub fn new() -> Self {
        Self { rotation_rate: 1.0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Turn {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq)]
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MoveAttributesCpt {
    pub speed: f32,
    pub turn_rate: f32,
}
impl MoveAttributesCpt {
    pub fn new() -> Self {
        Self {
            speed: 400.0,
            turn_rate: 10.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HumanInputCpt {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoxColliderCpt {
    pub w: f32,
    pub h: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircleColliderCpt {
    pub r: f32,
}
impl CircleColliderCpt {
    pub fn new() -> Self {
        Self { r: 10.0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TranslationalInputCpt {
    pub direction: Option<Direction>,
}
impl TranslationalInputCpt {
    pub fn new() -> Self {
        Self { direction: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProjectileEmitterCpt {
    pub projectile_velocity: nalgebra_glm::Vec2,
    pub cooldown: i32,
    pub projectile_duration: time::Duration,
    pub hit_damage: i32,
    pub is_friendly: bool,
    pub last_emission_time: Option<time::Instant>,
}

impl ProjectileEmitterCpt {
    pub fn new() -> Self {
        Self {
            projectile_velocity: Vec2::new(0., 0.),
            cooldown: 0,
            projectile_duration: time::Duration::new(10, 0),
            hit_damage: 10,
            is_friendly: false,
            last_emission_time: None,
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
