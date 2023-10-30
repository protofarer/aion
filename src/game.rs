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

use crate::archetypes::{
    gen_attached_orbiting_particle, gen_buncha_rng_particles, gen_unattached_orbiting_particle,
};
use crate::avatars::{Circloid, HumanShip};
use crate::draw::{draw_circle, draw_pixel, draw_rect};
use crate::gui::Framework;
use crate::pixel::*;
use crate::scenario::{
    gen_intersecting_particles, gen_row_particles, spawn_scenario1, spawn_scenario2,
};
use crate::time::{Dt, FrameTimer};
use crate::util::*;
use crate::{dev, game, log_error, LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH}; // little function in main.rs
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

        let ship = self.world.spawn(HumanShip::new());
        // spawn_scenario1(&mut self.world);
        spawn_scenario2(&mut self.world);

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
        // ai goes somewhere at the end and produces an input to be handled in next update tick
    }

    pub fn render(&mut self, pixels: &mut Pixels, dbg_ctx: &DebugContext) {
        if (self.get_runstate() != RunState::Running) && (self.get_runstate() != RunState::Paused) {
            return;
        }
        let mut frame = pixels.frame_mut();

        clear(frame);
        draw_boundary(frame);

        // draw avatars
        for (_id, (transform, drawbody)) in self.world.query_mut::<(&TransformCpt, &DrawBodyCpt)>()
        {
            draw_avatar(frame, transform, drawbody);
        }

        // draw orbiting particles
        // ? refactor this?, could be done in avatar render loop?
        // the transformcpt for a orbitparticle should be for the particle itself
        // orbit particle integration should handle correct transform update
        // the rigidbody should represent the actual particle itself... i think
        // is there a better way to do this?
        // transform is important for collisions..when this becomes a sort of orbitingprojectile
        // ... or
        // ... keep this and have collision detector calculate particle real position
        // ... or add an OrbitParticlePositionCpt... but then I am doing
        // collision area calculations differently than just using transformcpt?
        // * ... or maybe the OrbitParticleCpt HAS a position which is only used for the center of the orbit!
        // - then no need for rigidbody, add velocity to cpt itself
        // - (add in rigidbodycpt)
        // - re-use RotatableBodyCpt for the theta speed, perhaps add a MoveAttributes for speed
        // ? - is it best to define kinematics in terms of theta or tangential velocity?

        // orbiting particle showdown
        // - transform is actual particle position
        // - have a settable speed (easier to think about than angular_velocity) and r
        // - rigidbody velocity reflects above, is for actual particle
        // - can have a TransformParentCpt, whose entity the orbital center position will copy

        // for (_id, (transform, drawbody, orbiting_particle)) in
        //     self.world
        //         .query_mut::<(&TransformCpt, &DrawBodyCpt, &OrbitParticleCpt)>()
        // {
        //     draw_body_of_orbiting_particle(frame, transform, drawbody, orbiting_particle);
        // }

        if dbg_ctx.is_drawing_collisionareas {
            for (_id, (transform, collision_circle)) in self
                .world
                .query_mut::<(&TransformCpt, &CircleColliderCpt)>()
            {
                draw_collision_circle(frame, transform, collision_circle);
            }
        }
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
