use anyhow::{Context, Result};
use hecs::{PreparedQuery, With, Without, World};
use log::info;
use rand::prelude::*;
#[allow(warnings)]
use std::collections::HashMap;
use std::time::{self, Duration, Instant};

use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use crate::draw::{draw_circle, draw_pixel, draw_rect};
use crate::geom::*;
use crate::gui::Framework;
use crate::pixel::*;
use crate::time::{Dt, FrameTimer};
use crate::{dev, game, log_error, INIT_DT, LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH}; // little function in main.rs
use crate::{draw_bodies::*, DebugContext};
use nalgebra_glm::Vec2;
use pixels::{Pixels, SurfaceTexture};

use crate::components::*;
use crate::systems::*;
pub struct WindowDims {
    pub w: f32,
    pub h: f32,
}

#[derive(PartialEq, Eq, Hash)] // ? is Eq needed? what's it do?
enum ButtonState {
    Up,
    Down,
}

// Game Loop States
// eg:
// [init] => Stopped => [run] => Running => [input: pause] => Paused => [input: stop]
// => Stopped => [input: pause] => (no effect)Stopped => [input: start/resume] => Resuming => Running => [input: pause]
// => Paused => [input: unpause] => Running
#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Running, // Update and render
    Paused,  // No update
    Stopped, //  No update or render
    Exiting, // Signal event loop break
}
pub struct RunController(RunState);

impl RunController {
    pub fn new() -> Self {
        RunController(RunState::Stopped)
    }
    pub fn run(&mut self) {
        dev!("Game Running");
        self.0 = RunState::Running;
    }
    pub fn stop(&mut self) {
        dev!("Game Stopping");
        self.0 = RunState::Stopped;
    }
    pub fn pause(&mut self) {
        dev!("Game Pausing");
        self.0 = RunState::Paused;
    }
    pub fn exit(&mut self) {
        dev!("Game Exiting");
        self.0 = RunState::Exiting;
    }
    pub fn get_state(&self) -> RunState {
        self.0
    }
}

pub trait GetRunState {
    fn get_runstate(&self) -> RunState;
}

pub struct Game {
    pub loop_controller: RunController,
    pub input: WinitInputHelper,
    pub world: World,
}

impl GetRunState for Game {
    fn get_runstate(&self) -> RunState {
        self.loop_controller.get_state()
    }
}

impl Game {
    pub fn new() -> Result<Self, anyhow::Error> {
        dev!("INIT start");

        let loop_controller = RunController::new();
        let mut world = World::new();

        dev!("INIT fin");

        Ok(Self {
            loop_controller,
            input: WinitInputHelper::new(),
            world,
        })
    }

    pub fn setup(&mut self) {
        dev!("SETUP start");

        // PLAYER ENTITY
        // todo use tag
        let _ = self.world.spawn((
            HumanInputCpt {},
            TransformCpt::new(),
            RigidBodyCpt::new(),     // current velocity, used for physics
            RotatableBodyCpt::new(), // curent turn rate, used for physics
            MoveAttributesCpt::new(),
            CircleColliderCpt { r: 15.0 },
            ColorBodyCpt {
                primary: Color::RGB(0, 255, 0),
                secondary: Color::RGB(0, 0, 0),
            },
            RotationalInputCpt::new(),
            ProjectileEmitterCpt {
                projectile_velocity: Vec2::new(0., 0.),
                cooldown: 250,
                projectile_duration: Duration::new(0, 3000_000_000),
                hit_damage: 10,
                is_friendly: true,
                last_emission_time: None,
            },
        ));

        // spawn_buncha_particles(&mut self.world);
        // spawn_buncha_circles(&mut self.world);
        // spawn_buncha_squares(&mut self.world);

        // spawn incoming circloids
        self.world.spawn((
            TransformCpt {
                position: Vec2::new(LOGICAL_WINDOW_WIDTH - 100., 100.),
                heading: 0.,
                scale: Vec2::new(1.0, 1.0),
            },
            RigidBodyCpt {
                velocity: Vec2::new(0., 100.),
            },
            CircleColliderCpt { r: 30.0 },
            ColorBodyCpt {
                primary: Color::RGB(160, 160, 0),
                secondary: Color::RGB(0, 0, 0),
            },
        ));
        // self.world.spawn((
        //     TransformCpt {
        //         position: Vec2::new(LOGICAL_WINDOW_WIDTH - 100., 300.),
        //         heading: 0.,
        //         scale: Vec2::new(1.0, 1.0),
        //     },
        //     RigidBodyCpt {
        //         velocity: Vec2::new(0., -100.),
        //     },
        //     CircleColliderCpt { r: 30.0 },
        //     ColorBodyCpt {
        //         primary: Color::RGB(255, 0, 0),
        //         secondary: Color::RGB(0, 0, 0),
        //     },
        // ));

        // collide it
        self.world.spawn((
            TransformCpt {
                position: Vec2::new(LOGICAL_WINDOW_WIDTH - 100., 300.),
                heading: 0.,
                scale: Vec2::new(1.0, 1.0),
            },
            RigidBodyCpt {
                velocity: Vec2::new(0., -100.),
            },
            ProjectileCpt {
                is_friendly: false,
                hit_damage: 0,
                duration: time::Duration::new(5, 0),
                start_time: time::Instant::now(),
            },
            ParticleColliderCpt {},
            ColorBodyCpt {
                primary: Color::RGB(255, 0, 0),
                secondary: Color::RGB(0, 0, 0),
            },
        ));

        // persist it
        self.world.spawn((
            TransformCpt {
                position: Vec2::new(LOGICAL_WINDOW_WIDTH - 300., 300.),
                heading: 0.,
                scale: Vec2::new(1.0, 1.0),
            },
            RigidBodyCpt {
                velocity: Vec2::new(0., 100.),
            },
            ProjectileCpt {
                is_friendly: false,
                hit_damage: 0,
                duration: time::Duration::new(10, 0),
                start_time: time::Instant::now(),
            },
            ParticleColliderCpt {},
            ColorBodyCpt {
                primary: Color::RGB(0, 255, 255),
                secondary: Color::RGB(0, 0, 0),
            },
        ));

        self.loop_controller.run();
        dev!("SETUP fin");
    }

    pub fn process_input(&mut self, dbg_context: &mut DebugContext) {
        self.process_dbg_keys(dbg_context);

        // produce player keys from key events, later, input system (ecs)
        // processes player keys and mutates ship state

        // self.process_player_control_keys();
    }

    pub fn update(&mut self, dt: Dt) {
        // let update_schedule = Schedule::builder()
        //     .add_system(process_rotational_input_system())
        //     .add_system(process_translational_input_system())
        //     .flush()
        //     .add_system(update_positions_system())
        //     .flush()
        //     .add_system(circle_collision_system())
        //     .flush()
        //     .add_system(world_boundary_bounce_rect_system())
        //     .add_system(world_boundary_bounce_circle_system())
        //     .build();
        let runstate = self.get_runstate();
        system_process_ship_controls(&mut self.world, runstate, &self.input);
        system_integrate_rotation(&mut self.world, &dt);
        system_integrate_translation(&mut self.world, &dt);
        system_boundary_restrict_circle(&mut self.world);
        system_boundary_restrict_particle(&mut self.world);
        system_circle_collision(&mut self.world);
        system_particle_collision(&mut self.world);
    }

    pub fn render(&mut self, pixels: &mut Pixels, dbg_ctx: &DebugContext) {
        let mut frame = pixels.frame_mut();

        clear(frame);
        draw_boundary(frame);

        for (_id, (transform, collision_circle, colorbody)) in self
            .world
            .query_mut::<With<(&TransformCpt, &CircleColliderCpt, &ColorBodyCpt), &HumanInputCpt>>()
        {
            draw_ship_circle_collision(transform, collision_circle, colorbody, frame);
        }
        for (_id, (transform, collision_circle, colorbody)) in self
            .world
            .query_mut::<Without<(&TransformCpt, &CircleColliderCpt, &ColorBodyCpt), &HumanInputCpt>>()
        {
            draw_circloid(transform, collision_circle, colorbody, frame);
        }
        for (_id, (transform, colorbody)) in self
            .world
            .query_mut::<With<(&TransformCpt, &ColorBodyCpt), &ParticleColliderCpt>>()
        {
            draw_pixel(
                transform.position.x as i32,
                transform.position.y as i32,
                colorbody.primary,
                frame,
            );
        }

        if dbg_ctx.is_drawing_collisionareas {
            for (_id, (transform, collision_circle)) in self
                .world
                .query_mut::<(&TransformCpt, &CircleColliderCpt)>()
            {
                draw_collision_circle(transform, collision_circle, frame);
            }
        }
    }

    pub fn destroy(&self) {
        dev!("DESTROY game");
    }

    fn process_player_control_keys(&mut self) {
        // let mut query = <(&HumanInputCpt, &mut RotationalInputCpt)>::query();

        // let input = &self.input;
        // let runstate = self.get_runstate();

        // for (_human, mut rotational_input) in query.iter_mut(&mut self.world) {
        //     set_rotational_input(input, runstate, &mut rotational_input);
        // }

        // if self.input.key_pressed(VirtualKeyCode::Space)
        //     || self.input.key_held(VirtualKeyCode::Space)
        // {
        //     let mut query = <&mut CraftActionStateCpt>::query();
        //     for state in query.iter_mut(&mut self.world) {
        //         state.is_firing_primary = true;
        //     }
        // }
    }

    fn process_dbg_keys(&mut self, dbg_ctx: &mut DebugContext) {
        if self.input.key_pressed(VirtualKeyCode::P) {
            if self.get_runstate() == RunState::Running {
                self.loop_controller.pause();
            } else if self.get_runstate() == RunState::Paused {
                self.loop_controller.run();
            }
        }
        if self.input.key_pressed(VirtualKeyCode::Semicolon) {
            if self.get_runstate() == RunState::Stopped {
                self.loop_controller.run();
            } else if self.get_runstate() != RunState::Stopped {
                self.loop_controller.stop();
            }
        }
        if self.input.key_pressed(VirtualKeyCode::Grave) {
            dbg_ctx.is_on = !dbg_ctx.is_on;
        }
        if self.input.key_pressed(VirtualKeyCode::Key1) {
            dbg_ctx.is_drawing_collisionareas = !dbg_ctx.is_drawing_collisionareas;
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        dev!("Game dropped");
    }
}

fn clear(frame: &mut [u8]) {
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(BLACK.as_bytes());
    }
}

fn gen_particle() -> (TransformCpt, RigidBodyCpt, BoxColliderCpt, ColorBodyCpt) {
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
            velocity: Vec2::new(rng.gen::<f32>() * 1000.0, rng.gen::<f32>() * 1000.0),
        },
        BoxColliderCpt { w: 1.0, h: 1.0 },
        ColorBodyCpt {
            primary: GREY,
            secondary: Color::RGB(0, 0, 0),
        },
    )
}

// there's a rusty way to populate a vector
fn gen_particles(n: i32) -> Vec<(TransformCpt, RigidBodyCpt, BoxColliderCpt, ColorBodyCpt)> {
    let mut particles = vec![];
    for i in 0..n {
        particles.push(gen_particle());
    }
    particles
}

pub fn gen_boxoid() -> (TransformCpt, RigidBodyCpt, BoxColliderCpt, ColorBodyCpt) {
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
            velocity: Vec2::new(rng.gen::<f32>() * 500.0, rng.gen::<f32>() * 500.0),
        },
        BoxColliderCpt { w: 15.0, h: 15.0 },
        ColorBodyCpt {
            primary: RED,
            secondary: Color::RGB(0, 0, 0),
        },
    )
}
pub fn gen_boxoids(n: i32) -> Vec<(TransformCpt, RigidBodyCpt, BoxColliderCpt, ColorBodyCpt)> {
    let mut squares = vec![];
    for i in 0..n {
        squares.push(gen_boxoid());
    }
    squares
}

fn gen_circloid() -> (TransformCpt, RigidBodyCpt, CircleColliderCpt, ColorBodyCpt) {
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

fn spawn_buncha_particles(world: &mut World) {
    // let _: &[Entity] = world.extend(gen_particles(1000));
}

fn spawn_buncha_boxoids(world: &mut World) {
    // let _: &[Entity] = world.extend(gen_boxoids(12));
}

fn spawn_buncha_circloids(world: &mut World) {
    // let _: &[Entity] = world.extend(gen_circloids(5));
}

pub fn deg_to_rad(x: f32) -> f32 {
    x * nalgebra_glm::pi::<f32>() / 180.0
}
