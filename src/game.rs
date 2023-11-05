#[allow(warnings)]
use anyhow::{anyhow, Context, Result};
use audio_manager::SoundManager;
use hecs::{PreparedQuery, With, Without, World};
use log::info;
use rand::prelude::*;
use rodio::cpal::traits::HostTrait;
use rodio::{Decoder, DeviceTrait, OutputStream, OutputStreamHandle, Source};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::time::{self, Duration, Instant};

use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use crate::archetypes::{
    gen_attached_orbiting_particle, gen_buncha_rng_particles, gen_ping_animation,
    gen_unattached_orbiting_particle,
};
use crate::audio::{load_essential_sound_effects, SoundEffectNames};
use crate::avatars::{Circloid, HumanShip};
use crate::gfx::draw::{draw_arcs, draw_circle, draw_pixel, draw_rect};
use crate::gfx::draw_bodies::{draw_avatar, draw_boundary, draw_collision_circle};
use crate::gfx::pixel::*;
use crate::gui::Framework;
use crate::scenario::{
    gen_intersecting_particles, gen_row_particles, spawn_scenario1, spawn_scenario_shootingallery,
};
use crate::util::time::{Dt, FrameTimer};
use crate::{dev, game, log_error, DebugContext, LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH}; // little function in main.rs
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
    pub sound_manager: SoundManager,
}

impl GetRunState for Game {
    fn get_runstate(&self) -> RunState {
        self.loop_controller.get_state()
    }
}

impl Game {
    pub fn new() -> Result<Self, anyhow::Error> {
        dev!("INIT start");

        let mut sound_manager = SoundManager::new().map_err(|e| anyhow!("{}", e))?;

        dev!("INIT fin");

        Ok(Self {
            loop_controller: RunController::new(),
            input: WinitInputHelper::new(),
            world: World::new(),
            sound_manager,
        })
    }

    pub fn setup(&mut self) {
        dev!("SETUP start");

        if let Err(e) = load_essential_sound_effects(&mut self.sound_manager) {
            eprintln!("{e}");
        }

        let ship = self.world.spawn(HumanShip::new());

        // spawn_scenario1(&mut self.world);
        spawn_scenario_shootingallery(&mut self.world);

        // self.world.spawn(gen_unattached_orbiting_particle(
        //     300., 300., 100., 100., 25., 200., GREEN,
        // ));

        // self.world
        //     .spawn(gen_attached_orbiting_particle(ship, 35., 1000., GREEN));

        self.loop_controller.run();
        dev!("SETUP fin");
    }

    pub fn update(&mut self, dt: Dt) {
        let runstate = self.get_runstate();
        // if I moved this to game.process_input?
        if runstate != RunState::Running {
            return;
        }

        system_process_human_input(&mut self.world, runstate, &self.input);
        system_projectile_emission(&mut self.world);
        system_integrate_rotation(&mut self.world, &dt);
        system_integrate_translation(&mut self.world, &dt);
        system_integrate_orbiting_particles(&mut self.world, &dt);
        system_boundary_restrict_circloid(&mut self.world);
        system_boundary_restrict_particletypes(&mut self.world);
        test_system_boundary_restrict_particle(&mut self.world);
        system_collision_detection(&mut self.world);
        system_collision_resolution(&mut self.world);
        system_physical_damage_resolution(&mut self.world);
        system_sound_effects(&mut self.world, &self.sound_manager);
    }

    pub fn render(&mut self, pixels: &mut Pixels, dbg_ctx: &DebugContext, rdt: Dt) {
        if (self.get_runstate() != RunState::Running) && (self.get_runstate() != RunState::Paused) {
            return;
        }
        let mut frame = pixels.frame_mut();
        clear(frame);
        draw_boundary(frame);

        for (_id, (transform, drawbody)) in self.world.query_mut::<(&TransformCpt, &DrawBodyCpt)>()
        {
            draw_avatar(frame, transform, drawbody);
        }

        system_render_pings(&mut self.world, &mut frame);
        system_animation_lifecycle(&mut self.world, rdt);

        if dbg_ctx.is_drawing_collisionareas {
            for (_id, (transform, collision_circle)) in self
                .world
                .query_mut::<(&TransformCpt, &CircleColliderCpt)>()
            {
                draw_collision_circle(frame, transform, collision_circle);
            }
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        dev!("Game dropped");
    }
}
