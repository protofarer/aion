use nalgebra_glm::Vec2;

use crate::pixel::Color;
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub position: Vec2,
    pub rotation: f32,
    pub scale: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RigidBody {
    pub velocity: Vec2,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovementStats {
    pub speed: f32,
    pub turn_rate: f32,
    pub decel: f32,
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
pub struct TranslationalInput {
    pub direction: Option<Direction>,
}
