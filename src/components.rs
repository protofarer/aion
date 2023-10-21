use crate::pixel::{Color, BLUE, WHITE};
use nalgebra_glm::Vec2;
use std::time;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}
impl Transform {
    fn new() -> Self {
        Self {
            position: Vec2::new(100., 100.),
            rotation: 0.,
            scale: Vec2::new(1., 1.),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RigidBody {
    pub velocity: Vec2,
}
impl RigidBody {
    fn new() -> Self {
        Self {
            velocity: Vec2::new(100., 0.),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CollisionArea {
    pub w: f32,
    pub h: f32,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ColorBody {
    pub primary: Color,
    pub secondary: Color,
}
impl ColorBody {
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
pub struct RotationalInput {
    pub turn_sign: Option<Turn>,
    pub is_thrusting: bool,
}
impl RotationalInput {
    fn new() -> Self {
        Self {
            turn_sign: None,
            is_thrusting: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovementStats {
    pub speed: f32,
    pub turn_rate: f32,
}
impl MovementStats {
    fn new() -> Self {
        Self {
            speed: 100.0,
            turn_rate: 0.1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TranslationalInput {
    pub direction: Option<Direction>,
}
impl TranslationalInput {
    fn new() -> Self {
        Self { direction: None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CraftState {
    pub is_firing_primary: bool,
    pub is_firing_secondary: bool,
}
impl CraftState {
    fn new() -> Self {
        Self {
            is_firing_primary: false,
            is_firing_secondary: false,
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
pub struct ProjectileEmitterComponent {
    projectile_velocity: nalgebra_glm::Vec2,
    repeat_frequency: i32,
    projectile_duration: i32,
    hit_damage: i32,
    is_friendly: bool,
    last_emission_time: time::Instant,
}

impl ProjectileEmitterComponent {
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
