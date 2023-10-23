use crate::pixel::{Color, BLUE, WHITE};
use nalgebra_glm::Vec2;
use std::time;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransformCpt {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}
impl TransformCpt {
    fn new() -> Self {
        Self {
            position: Vec2::new(100., 100.),
            rotation: 0.,
            scale: Vec2::new(1., 1.),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RigidBodyCpt {
    pub velocity: Vec2,
}
impl RigidBodyCpt {
    fn new() -> Self {
        Self {
            velocity: Vec2::new(100., 0.),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoxColliderCpt {
    pub w: f32,
    pub h: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CircleColliderCpt {
    pub r: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorBodyCpt {
    pub primary: Color,
    pub secondary: Color,
}
impl ColorBodyCpt {
    fn new() -> Self {
        Self {
            primary: WHITE,
            secondary: BLUE,
        }
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
    fn new() -> Self {
        Self {
            turn_sign: None,
            is_thrusting: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovementStatsCpt {
    pub speed: f32,
    pub turn_rate: f32,
}
impl MovementStatsCpt {
    fn new() -> Self {
        Self {
            speed: 100.0,
            turn_rate: 0.1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HumanInputCpt {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TranslationalInputCpt {
    pub direction: Option<Direction>,
}
impl TranslationalInputCpt {
    fn new() -> Self {
        Self { direction: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CraftActionStateCpt {
    pub is_firing_primary: bool,
    pub is_firing_secondary: bool,
}
impl CraftActionStateCpt {
    fn new() -> Self {
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
    fn new() -> Self {
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
pub struct ProjectileEmitter {
    projectile_velocity: nalgebra_glm::Vec2,
    repeat_frequency: i32,
    projectile_duration: i32,
    hit_damage: i32,
    is_friendly: bool,
    last_emission_time: time::Instant,
}

impl ProjectileEmitter {
    fn new() -> Self {
        Self {
            projectile_velocity: Vec2::new(0., 0.),
            repeat_frequency: 0,
            projectile_duration: 10000,
            hit_damage: 10,
            is_friendly: false,
            last_emission_time: time::Instant::now(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Projectile {
    is_friendly: bool,
    hit_damage: i32,
    duration: time::Instant,
    start_time: time::Instant,
}
impl Projectile {
    fn new() -> Self {
        Self {
            is_friendly: false,
            hit_damage: 0,
            duration: time::Instant::now(),
            start_time: time::Instant::now(),
        }
    }
}
