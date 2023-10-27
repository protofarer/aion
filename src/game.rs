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

use crate::archetypes::gen_buncha_rng_particles;
use crate::avatars::{Circloid, HumanShip};
use crate::draw::{draw_circle, draw_pixel, draw_rect};
use crate::gui::Framework;
use crate::pixel::*;
use crate::scenario::{gen_intersecting_particles, gen_row_particles, spawn_scenario1};
use crate::time::{Dt, FrameTimer};
use crate::util::*;
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

    pub fn process_input(&mut self, dbg_context: &mut DebugContext) {
        self.process_dbg_keys(dbg_context);

        // produce player keys from key events, later, input system (ecs)
        // processes player keys and mutates ship state

        // self.process_player_control_keys();
    }

    pub fn setup(&mut self) {
        dev!("SETUP start");

        let _ = self.world.spawn(HumanShip::new());
        // self.world.spawn(Circloid::new());
        spawn_scenario1(&mut self.world);

        // todo generate a good explorative scenario
        // - noncolliding every color particle, bounce vertical (color tweaks)
        // - 1 pair pass-thru particles
        // - noncolliding & colliding projectile, bounce vertical
        // - noncolliding & colliding circloids, bounce vertical
        // - noncolliding & colliding ships, bounce vertical

        // self.world.spawn_batch(
        //     gen_buncha_rng_particles(1000)
        //         .into_iter()
        //         .map(|arch_particle| arch_particle.into_tuple()),
        // );
        // self.world.spawn_batch(gen_row_particles());
        // self.world
        //     .spawn_batch(gen_passing_particles().into_iter().map(|p| p.into_tuple()));

        self.loop_controller.run();
        dev!("SETUP fin");
    }

    pub fn update(&mut self, dt: Dt) {
        let runstate = self.get_runstate();
        system_process_human_input(&mut self.world, runstate, &self.input);
        system_integrate_rotation(&mut self.world, &dt);
        system_integrate_translation(&mut self.world, &dt);
        system_boundary_restrict_circle(&mut self.world);
        system_boundary_restrict_projectile(&mut self.world);
        system_boundary_restrict_particle(&mut self.world);
        system_circle_collision(&mut self.world);
        system_particle_collision(&mut self.world);
    }

    pub fn render(&mut self, pixels: &mut Pixels, dbg_ctx: &DebugContext) {
        let mut frame = pixels.frame_mut();

        clear(frame);
        draw_boundary(frame);

        // draw avatars
        for (_id, (transform, drawbody)) in self.world.query_mut::<(&TransformCpt, &DrawBodyCpt)>()
        {
            draw_avatar(transform, drawbody, frame);
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
