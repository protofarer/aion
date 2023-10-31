use crate::{
    audio::SoundEffectName,
    pixel::{Color, BLUE, CYAN, GREEN, GREY, MAGENTA, ORANGE, RED, WHITE, YELLOW},
    LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH,
};
use hecs::Entity;
use nalgebra_glm::Vec2;
use std::{default, time};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct TransformCpt {
    pub position: Vec2,
    pub heading: Theta,
    pub scale: Vec2,
}
impl TransformCpt {
    pub fn new() -> Self {
        Self {
            position: Vec2::new(LOGICAL_WINDOW_WIDTH / 2., LOGICAL_WINDOW_HEIGHT / 2.),
            heading: Theta::new(),
            scale: Vec2::new(1., 1.),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Theta(f32);
impl Theta {
    pub fn new() -> Self {
        Theta(0.)
    }
    pub fn unit_x(&self) -> f32 {
        (self.0).cos()
    }
    pub fn unit_y(&self) -> f32 {
        (self.0).sin()
    }
    pub fn d_degrees(&mut self, deg: f32) {
        let rad = deg * nalgebra_glm::pi::<f32>() / 180.0;
        self.0 = (self.0 + rad) % nalgebra_glm::two_pi::<f32>()
    }
    pub fn cos(&self) -> f32 {
        (self.0).cos()
    }
    pub fn sin(&self) -> f32 {
        (self.0).sin()
    }
    pub fn get(&self) -> f32 {
        self.0
    }
    pub fn set(&mut self, x: f32) {
        self.0 = x;
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
            turn_rate: 10.0,
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrbitParticleCpt {
    pub r: f32,
    pub speed: f32,
    pub angle: Theta,
    pub attached_to: Option<Entity>,
}

impl OrbitParticleCpt {
    pub fn new() -> Self {
        Self {
            r: 10.,
            speed: 50.,
            angle: Theta::new(),
            attached_to: None,
        }
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
    pub hit_damage: i32, // ? better as a DamageOnCollisionCpt ?
    pub is_friendly: bool,
    pub last_emission_time: time::Instant,
    pub intends_to_fire: bool,
}

impl ProjectileEmitterCpt {
    pub fn new() -> Self {
        Self {
            projectile_speed: 300.,
            cooldown: 100,
            projectile_duration: time::Duration::new(7, 0),
            hit_damage: 50,
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

#[derive(Clone, Copy, Debug)]
pub struct PhysicalDamageEvent {
    pub receiver: Entity,
    pub damage: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HealthCpt {
    pub hp: i32,
}
impl HealthCpt {
    pub fn new() -> Self {
        HealthCpt { hp: 100 }
    }
}

pub struct AnimationCpt {
    pub frame_count: usize,
    pub current_frame: usize,
    pub rfps: f32,
    pub rdt_accum: f32,
    pub repeat_count: i32,
    pub is_infinite_repeat: bool,
}
impl AnimationCpt {
    pub fn new(frame_count: usize) -> Self {
        AnimationCpt {
            frame_count,
            current_frame: 0,
            rfps: 1.,
            rdt_accum: 0.,
            repeat_count: 3,
            is_infinite_repeat: false,
        }
    }
}

pub struct PingDrawCpt {
    pub gap_factors: [i32; 4],
    pub r: f32,
}

pub struct SoundEffectEvent {
    pub name: SoundEffectName, // the const
}
