use anyhow::{Context, Result};
use log::info;
use rand::prelude::*;
#[allow(warnings)]
use std::collections::HashMap;
use std::time::{Duration, Instant};

use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use crate::draw::draw_circle;
use crate::draw_bodies::*;
use crate::geom::*;
use crate::gui::Framework;
use crate::pixel::*;
use crate::time::Dt;
use crate::{dev, game, log_error, INIT_DT, LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH}; // little function in main.rs
use legion::*;
use nalgebra_glm::Vec2;
use pixels::{Pixels, SurfaceTexture};

use super::systems::*;
use crate::components::*;
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
#[derive(PartialEq)]
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
    pub fn get_state(&self) -> &RunState {
        &self.0
    }
}

pub trait GetRunState {
    fn get_runstate(&self) -> &RunState;
}

pub struct Game {
    pub loop_controller: RunController,
    is_debug_on: bool,
    world: World,
    update_schedule: Schedule,
    resources: Resources,
    key_states: HashMap<VirtualKeyCode, Option<ButtonState>>,
    pub pixels: Pixels,
    pub input: WinitInputHelper,
}

impl GetRunState for Game {
    fn get_runstate(&self) -> &RunState {
        self.loop_controller.get_state()
    }
}

impl Game {
    pub fn new(pixels: Pixels) -> Result<Self, anyhow::Error> {
        dev!("INIT start");

        // todo 1. pass config struct
        // todo 2. let game init/new parse readline
        // todo 3. pass both, then readline args override config struct
        // todo 4. read a toml config file that can be overriden by readline

        let world = World::default();
        let mut resources = Resources::default();

        // ? is this correct, to send a ref to a copy trait variable
        resources.insert(WindowDims {
            w: LOGICAL_WINDOW_WIDTH,
            h: LOGICAL_WINDOW_HEIGHT,
        });

        let update_schedule = Schedule::builder()
            .add_system(process_rotational_input_system())
            .add_system(process_translational_input_system())
            .flush()
            .add_system(update_positions_system())
            .flush()
            .add_system(collision_system())
            .build();

        // let render_schedule = Schedule::builder().add_system(render_system()).build();

        let loop_controller = RunController::new();
        dev!("INIT fin");

        Ok(Self {
            loop_controller,
            is_debug_on: false,
            world,
            update_schedule,
            resources,
            key_states: HashMap::new(),
            pixels,
            input: WinitInputHelper::new(),
        })
    }

    pub fn setup(&mut self) {
        dev!("SETUP start");

        // PLAYER ENTITY
        // todo use tag
        let _ = self.world.push((
            Transform {
                position: Vec2::new(300.0, 300.0),
                rotation: 0.0,
                scale: Vec2::new(1.0, 1.0),
            },
            RigidBody {
                velocity: Vec2::new(0.0, 0.0),
            },
            CollisionArea { w: 20.0, h: 20.0 },
            ColorBody {
                primary: Color::RGB(0, 255, 0),
                secondary: Color::RGB(0, 0, 0),
            },
            RotationalInput {
                turn_sign: None,
                is_thrusting: false,
            },
            MovementStats {
                speed: 500f32,
                turn_rate: 0.1f32,
                decel: 0.5f32,
            },
        ));

        // BATCH ADD ENTS
        let _: &[Entity] = self.world.extend(gen_squares(12));
        let _: &[Entity] = self.world.extend(gen_particles(1000));
        let _: &[Entity] = self.world.extend(gen_circles(3));

        self.resources.insert(INIT_DT);
        dev!("SETUP fin");
    }

    fn process_player_control_keys(&mut self) {
        let input = &self.input;
        self.set_rotational_input();
    }

    fn set_rotational_input(&mut self) {
        let input = &self.input;
        fn set_input_turn(turn: Option<Turn>, world: &mut World) {
            let mut query = <&mut RotationalInput>::query();
            for input in query.iter_mut(world) {
                input.turn_sign = turn;
            }
        }
        fn set_input_thrust(is_thrusting: bool, world: &mut World) {
            let mut query = <&mut RotationalInput>::query();
            for input in query.iter_mut(world) {
                input.is_thrusting = is_thrusting;
            }
        }

        if *self.loop_controller.get_state() == RunState::Running {
            // HANDLE SINGLE MOVE KEYS
            if input.key_pressed(VirtualKeyCode::D) || input.key_held(VirtualKeyCode::D) {
                set_input_turn(Some(Turn::Right), &mut self.world);
            }
            if input.key_pressed(VirtualKeyCode::A) || input.key_held(VirtualKeyCode::A) {
                set_input_turn(Some(Turn::Left), &mut self.world);
            }
            if input.key_pressed(VirtualKeyCode::W) || input.key_held(VirtualKeyCode::W) {
                set_input_thrust(true, &mut self.world);
            }
        }

        // HANDLE KEY UPS
        if input.key_released(VirtualKeyCode::D) || input.key_released(VirtualKeyCode::A) {
            set_input_turn(None, &mut self.world);
        }
        if input.key_released(VirtualKeyCode::W) {
            set_input_thrust(false, &mut self.world);
        }
    }

    pub fn process_input(&mut self) {
        let mut input = &self.input;
        if input.key_pressed(VirtualKeyCode::Escape) {
            if *self.get_runstate() != RunState::Stopped {
                self.loop_controller.stop();
            }
        }
        if input.key_pressed(VirtualKeyCode::P) {
            if *self.get_runstate() == RunState::Running {
                self.loop_controller.pause();
            } else if *self.get_runstate() == RunState::Paused {
                self.loop_controller.run();
            }
        }
        if input.key_pressed(VirtualKeyCode::Semicolon) {
            if *self.get_runstate() == RunState::Stopped {
                self.loop_controller.run();
            } else if *self.get_runstate() != RunState::Stopped {
                self.loop_controller.stop();
            }
        }
        if input.key_pressed(VirtualKeyCode::Grave) {
            self.is_debug_on = !self.is_debug_on;
        }
        self.process_player_control_keys();
    }

    pub fn update(&mut self, dt: Dt) {
        self.resources.insert(dt);
        self.update_schedule
            .execute(&mut self.world, &mut self.resources);
    }

    pub fn draw(&mut self) {
        let mut frame = self.pixels.frame_mut();
        clear(frame);

        draw_boundary(frame);

        let mut query = <(&Transform, &CollisionArea, &ColorBody, &RotationalInput)>::query();

        for (transform, collision_area, colorbody, rotational) in query.iter(&self.world) {
            draw_ship(transform, collision_area, colorbody, frame);
        }

        // let mut query = <(&Transform, &CollisionArea, &ColorBody)>::query()
        //     .filter(!component::<RotationalInput>());
        // for (transform, _collision_area, colorbody) in query.iter(&self.world) {
        // }

        let mut query = <(&Transform, &CollisionArea, &ColorBody)>::query()
            .filter(!component::<RotationalInput>());
        for (transform, ca, colorbody) in query.iter(&self.world) {
            if ca.w == 1. {
                draw_particle(transform, colorbody, frame);
            } else if ca.w == 60. {
                draw_circloid(transform, ca, colorbody, frame);
            } else {
                draw_box(transform, colorbody, frame);
            }
        }

        // black hole
        draw_circle(frame, 200, 350, 60, WHITE);

        // star
        draw_circle(frame, 800, 100, 40, ORANGE);
    }

    pub fn destroy(&self) {
        dev!("DESTROY game");
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        dev!("Game dropped");
    }
}

fn clear(frame: &mut [u8]) {
    // for (i, byte) in frame.iter_mut().enumerate() {
    //     *byte = if i % 4 == 3 { 255 } else { 0 };
    // }
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(BLACK.as_bytes());
    }
}

fn gen_particle() -> (Transform, RigidBody, CollisionArea, ColorBody) {
    let mut rng = rand::thread_rng();
    (
        Transform {
            position: Vec2::new(
                rng.gen::<f32>() * LOGICAL_WINDOW_WIDTH,
                rng.gen::<f32>() * LOGICAL_WINDOW_HEIGHT,
            ),
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
        },
        RigidBody {
            velocity: Vec2::new(rng.gen::<f32>() * 1000.0, rng.gen::<f32>() * 1000.0),
        },
        CollisionArea { w: 1.0, h: 1.0 },
        ColorBody {
            primary: GREY,
            secondary: Color::RGB(0, 0, 0),
        },
    )
}
// there's a rusty way to populate a vector
fn gen_particles(n: i32) -> Vec<(Transform, RigidBody, CollisionArea, ColorBody)> {
    let mut particles = vec![];
    for i in 0..n {
        particles.push(gen_particle());
    }
    particles
}
fn gen_square() -> (Transform, RigidBody, CollisionArea, ColorBody) {
    let mut rng = rand::thread_rng();
    (
        Transform {
            position: Vec2::new(
                rng.gen::<f32>() * LOGICAL_WINDOW_WIDTH,
                rng.gen::<f32>() * LOGICAL_WINDOW_HEIGHT,
            ),
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
        },
        RigidBody {
            velocity: Vec2::new(rng.gen::<f32>() * 500.0, rng.gen::<f32>() * 500.0),
        },
        CollisionArea { w: 25.0, h: 25.0 },
        ColorBody {
            primary: RED,
            secondary: Color::RGB(0, 0, 0),
        },
    )
}
fn gen_squares(n: i32) -> Vec<(Transform, RigidBody, CollisionArea, ColorBody)> {
    let mut squares = vec![];
    for i in 0..n {
        squares.push(gen_square());
    }
    squares
}

fn gen_circle() -> (Transform, RigidBody, CollisionArea, ColorBody) {
    let mut rng = rand::thread_rng();
    (
        Transform {
            position: Vec2::new(
                rng.gen::<f32>() * LOGICAL_WINDOW_WIDTH,
                rng.gen::<f32>() * LOGICAL_WINDOW_HEIGHT,
            ),
            rotation: 0.0,
            scale: Vec2::new(1.0, 1.0),
        },
        RigidBody {
            velocity: Vec2::new(rng.gen::<f32>() * 100.0, rng.gen::<f32>() * 100.0),
        },
        CollisionArea { w: 60.0, h: 60.0 },
        ColorBody {
            primary: Color::RGB(160, 160, 0),
            secondary: Color::RGB(0, 0, 0),
        },
    )
}
fn gen_circles(n: i32) -> Vec<(Transform, RigidBody, CollisionArea, ColorBody)> {
    let mut circles = vec![];
    for i in 0..n {
        circles.push(gen_circle());
    }
    circles
}
