use anyhow::{Context, Result};
use log::info;
#[allow(warnings)]
use std::collections::HashMap;
use std::time::{Duration, Instant};
use winit::event_loop::EventLoop;

use winit::event::{Event, VirtualKeyCode};
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use crate::draw::{draw_line, draw_pixel};
use crate::gui::Framework;
use crate::pixel::Color;
use crate::{dev, game, log_error, WINDOW_HEIGHT, WINDOW_WIDTH}; // little function in main.rs
use legion::*;
use nalgebra_glm::Vec2;
use pixels::{Pixels, SurfaceTexture};

use super::systems::*;
use crate::components::*;

const FRAMERATE: u8 = 60;
const FRAME_LIMIT_MS: f32 = 1000.0 / FRAMERATE as f32;

#[derive(Debug)]
pub enum InitError {
    SDLInitFailed,
    VideoSubsystemFailed,
    WindowCreationFailed,
    CanvasCreationFailed,
    EventPumpFailure,
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Initialization Error: {:?}", self)
    }
}

impl std::error::Error for InitError {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dt(pub f32);
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
pub enum LoopState {
    Running, // Update and render
    Paused,  // No update
    Stopped, //  No update or render
    Exiting, // Signal event loop break
}
pub struct LoopController(LoopState);

impl LoopController {
    pub fn new() -> Self {
        LoopController(LoopState::Stopped)
    }
    pub fn run(&mut self) {
        dev!("Loop Running");
        self.0 = LoopState::Running;
    }
    pub fn stop(&mut self) {
        dev!("Loop Stopping");
        self.0 = LoopState::Stopped;
    }
    pub fn pause(&mut self) {
        dev!("Loop Pausing");
        self.0 = LoopState::Paused;
    }
    pub fn exit(&mut self) {
        dev!("Loop Exiting");
        self.0 = LoopState::Exiting;
    }
    pub fn get_state(&self) -> &LoopState {
        &self.0
    }
}

pub trait GetLoopState {
    fn get_loopstate(&self) -> &LoopState;
}

pub struct Game {
    pub loop_controller: LoopController,
    is_debug_on: bool,
    world: World,
    update_schedule: Schedule,
    resources: Resources,
    key_states: HashMap<VirtualKeyCode, Option<ButtonState>>,
}

impl GetLoopState for Game {
    fn get_loopstate(&self) -> &LoopState {
        self.loop_controller.get_state()
    }
}

impl Game {
    pub fn new() -> Result<Self, anyhow::Error> {
        dev!("INIT start");

        // todo 1. pass config struct
        // todo 2. let game init/new parse readline
        // todo 3. pass both, then readline args override config struct
        // todo 4. read a toml config file that can be overriden by readline

        let world = World::default();
        let mut resources = Resources::default();

        // ? is this correct, to send a ref to a copy trait variable
        resources.insert(WindowDims {
            w: WINDOW_WIDTH,
            h: WINDOW_HEIGHT,
        });

        // .add_system(process_input_system())
        let update_schedule = Schedule::builder()
            .flush()
            .add_system(update_movement_state_system())
            .add_system(update_positions_system())
            .flush()
            .add_system(collision_system())
            .build();

        // let render_schedule = Schedule::builder().add_system(render_system()).build();

        let loop_controller = LoopController::new();
        dev!("INIT fin");

        Ok(Self {
            loop_controller,
            is_debug_on: false,
            world,
            update_schedule,
            resources,
            key_states: HashMap::new(),
        })
    }

    pub fn process_input(&mut self, input: &WinitInputHelper) {
        if input.key_pressed(VirtualKeyCode::Escape) {
            if *self.get_loopstate() != LoopState::Stopped {
                self.loop_controller.stop();
            }
        }
        if input.key_pressed(VirtualKeyCode::P) {
            if *self.get_loopstate() == LoopState::Running {
                self.loop_controller.pause();
            } else if *self.get_loopstate() == LoopState::Paused {
                self.loop_controller.run();
            }
        }
        if input.key_pressed(VirtualKeyCode::Semicolon) {
            if *self.get_loopstate() == LoopState::Stopped {
                self.loop_controller.run();
            } else if *self.get_loopstate() != LoopState::Stopped {
                self.loop_controller.stop();
            }
        }
        self.process_player_control_keys(input);
        // self.handle_key_up(keys_down);
    }

    // pub fn handle_tick(&mut self, ms_prev_frame: &Instant) {
    //     let time_to_wait: f32 =
    //         FRAME_LIMIT_MS - Instant::now().duration_since(*ms_prev_frame).as_millis() as f32;

    //     // fixed frame rate: if below threshold MILLISECS_PER_FRAME then sleep
    //     if time_to_wait > 0.0 && time_to_wait <= FRAME_LIMIT_MS {
    //         let sleep_duration = Duration::new(0, (time_to_wait * 1000000.0) as u32);

    //         // * Sleeping for sleep_duration tends to actually sleep a little over by ~0.070 ms
    //         // let presleep = Instant::now();
    //         ::std::thread::sleep(sleep_duration);
    //         // let sleep_dur = Instant::now().duration_since(presleep);
    //         // println!("sleep_thread_duration(ms): {}", sleep_dur.as_micros() as f32 / 1000f32);

    //         // println!("ttw: {} sleep_for: {}", time_to_wait, sleep_duration.as_micros() as f64 /1000.0);
    //         if sleep_duration.as_millis() <= 2 {
    //             // Logger::info(&format!(
    //             // "Frames getting tight: sleeping {:?}ms",
    //             // sleep_duration.as_millis()
    //             // ));
    //         }
    //     }

    //     // aka elapsed seconds for a frame
    //     let dt = Dt(Instant::now().duration_since(*ms_prev_frame).as_secs_f32());
    //     self.resources.insert(dt.clone());
    //     // println!("dt_in_handletick: {}_sec", dt.0);

    //     self.fps_queue.push(1_f32 / dt.0); // dt to millis is u128
    //     self.fps = self.fps_queue.avg().unwrap_or(0f32);
    // }

    pub fn setup(&mut self) {
        dev!("SETUP start");

        // PLAYER ENTITY
        let _ = self.world.push((
            Transform {
                position: Vec2::new(300.0, 300.0),
                rotation: 0.0,
                scale: Vec2::new(1.0, 1.0),
            },
            RigidBody {
                velocity: Vec2::new(0.0, 0.0),
            },
            CollisionArea { w: 50.0, h: 50.0 },
            ColorBody {
                primary: Color::RGB(0, 255, 0),
                secondary: Color::RGB(0, 0, 0),
            },
            RotationalInput {
                turn_sign: None,
                is_thrusting: false,
            },
            MovementStats {
                speed: 10f32,
                turn_rate: 0.1f32,
                decel: 0.2f32,
            },
        ));

        // BATCH ADD ENTS
        // let _: &[Entity] = self.world.extend(vec![
        //     (
        //         Transform {
        //             position: Vec2::new(200.0, 100.0),
        //             rotation: 0.0,
        //             scale: Vec2::new(1.0, 1.0),
        //         },
        //         RigidBody {
        //             velocity: Vec2::new(200.0, 100.0),
        //         },
        //         CollisionArea { w: 50, h: 50 },
        //         ColorBody {
        //             primary: Color::RGB(255, 0, 0),
        //             secondary: Color::RGB(0, 0, 0),
        //         },
        //     ),
        //     (
        //         Transform {
        //             position: Vec2::new(400.0, 100.0),
        //             rotation: 0.0,
        //             scale: Vec2::new(1.0, 1.0),
        //         },
        //         RigidBody {
        //             velocity: Vec2::new(200.0, 100.0),
        //         },
        //         CollisionArea { w: 50, h: 50 },
        //         ColorBody {
        //             primary: Color::RGB(0, 0, 255),
        //             secondary: Color::RGB(0, 0, 0),
        //         },
        //     ),
        // ]);

        self.resources.insert(Dt(0.01667f32));
        dev!("SETUP fin");
    }

    pub fn update(&mut self) {
        self.update_schedule
            .execute(&mut self.world, &mut self.resources);
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        let mut query = <(&Transform, &CollisionArea, &ColorBody)>::query();

        for (transform, _collision_area, colorbody) in query.iter(&self.world) {
            draw_ship(transform, colorbody, frame);
            // self.canvas
            //     .fill_rect(Rect::new(
            //         transform.position.x as i32,
            //         transform.position.y as i32,
            //         collision_area.w,
            //         collision_area.h,
            //     ))
            //     .expect("FAIL canvas.fill_rect");
        }
        // for i in 25..500 {
        //     draw_pixel(i, 300, Color::RGB(0, 255, 0), frame);
        // }
        // draw_pixel(800, 800, Color::RGB(0, 255, 0), frame);
        // draw_line(25, 25, 500, 500, Color::RGB(255, 0, 0), frame);
    }

    pub fn destroy(&self) {
        // Logger::dbg("Destroy game");
    }

    fn process_player_control_keys(&mut self, input: &WinitInputHelper) {
        // HANDLE VALID SIMULTANEOUS MOVE KEYS
        // let key_downs: Vec<VirtualKeyCode> = self
        //     .key_states
        //     .iter()
        //     .filter_map(|(keycode, buttonstate)| {
        //         if *buttonstate == Some(ButtonState::Down) {
        //             Some(*keycode)
        //         } else {
        //             None
        //         }
        //     })
        //     .collect();
        self.set_rotational_input(input);
        //     // self.set_translational_input(key_downs);

        //     // HANDLE KEY UPS
        //     let key_ups: Vec<Keycode> = self
        //         .key_states
        //         .iter()
        //         .filter_map(|(keycode, buttonstate)| {
        //             if *buttonstate == Some(ButtonState::Up) {
        //                 Some(*keycode)
        //             } else {
        //                 None
        //             }
        //         })
        //         .collect();

        //     let mut key_ups_to_remove: Vec<Keycode> = vec![];

        //     for keycode in key_ups.iter() {
        //         match keycode {
        //             // HANDLE GAME RUN STATE KEYS
        //             Keycode::P => {
        //                 match self.run_state {
        //                     RunState::Paused => {
        //                         Logger::info("Game unpaused");
        //                         self.run_state = RunState::Running;
        //                     }
        //                     RunState::Running => {
        //                         Logger::info("Game paused");
        //                         self.run_state = RunState::Paused;
        //                     }
        //                     _ => {}
        //                 }
        //                 key_ups_to_remove.push(Keycode::P);
        //             }
        //             Keycode::Semicolon => {
        //                 match self.run_state {
        //                     RunState::Stopped => {
        //                         Logger::info("Game resuming");
        //                         self.run_state = RunState::Resuming;
        //                     }
        //                     RunState::Paused | RunState::Running => {
        //                         Logger::info("Game stopped");
        //                         self.run_state = RunState::Stopped;
        //                     }
        //                     _ => {
        //                         Logger::dbg("Cannot stop game while it is in process of resuming");
        //                     }
        //                 }
        //                 key_ups_to_remove.push(Keycode::Semicolon);
        //             }
        //             Keycode::Quote => {
        //                 self.is_debug_on = !self.is_debug_on;
        //                 let mode = if self.is_debug_on { "ON" } else { "OFF" };
        //                 key_ups_to_remove.push(Keycode::Quote);
        //                 Logger::dbg(&format!("Debug mode {}", mode));
        //             }
        //             Keycode::D | Keycode::A => {
        //                 // same code as fn set_rotational_input somewhere else
        //                 let mut query = <&mut RotationalInput>::query();
        //                 for input in query.iter_mut(&mut self.world) {
        //                     input.turn_sign = None;
        //                 }
        //                 key_ups_to_remove.push(*keycode);
        //             }
        //             Keycode::W => {
        //                 // same code as fn set_rotational_input somewhere else
        //                 let mut query = <&mut RotationalInput>::query();
        //                 for input in query.iter_mut(&mut self.world) {
        //                     input.is_thrusting = false;
        //                 }
        //                 key_ups_to_remove.push(*keycode);
        //             }
        //             Keycode::S => {
        //                 // let mut query = <&mut TranslationalInput>::query();
        //                 // for input in query.iter_mut(&mut self.world) {
        //                 //     input.direction = None;
        //                 // }

        //                 key_ups_to_remove.push(*keycode);
        //             }
        //             otherkeycodes => {
        //                 // Defaults to remove from hashmap, cleanup keys w/o associated action
        //                 key_ups_to_remove.push(*otherkeycodes);
        //             }
        //         }
        //     }
        //     for keycode in key_ups_to_remove {
        //         self.key_states.remove(&keycode);
        //     }
        // }

        // fn set_translational_input(&mut self, key_downs: Vec<Keycode>) {
        //     let mut set_input_dir = |dir: Direction| {
        //         let mut query = <&mut TranslationalInput>::query();
        //         for input in query.iter_mut(&mut self.world) {
        //             input.direction = Some(dir);
        //         }
        //     };

        //     if let RunState::Running = self.run_state {
        //         // ONLY ACTIVATE FOR TRANSLATIONAL HUMAN INPUTS... query the "player" input type
        //         if key_downs.contains(&Keycode::W) && key_downs.contains(&Keycode::D) {
        //             set_input_dir(Direction::NE)
        //         } else if key_downs.contains(&Keycode::S) && key_downs.contains(&Keycode::D) {
        //             set_input_dir(Direction::SE);
        //         } else if key_downs.contains(&Keycode::S) && key_downs.contains(&Keycode::A) {
        //             set_input_dir(Direction::SW);
        //         } else if key_downs.contains(&Keycode::W) && key_downs.contains(&Keycode::A) {
        //             set_input_dir(Direction::NW);
        //         } else {
        //             // HANDLE SINGLE MOVE KEYS
        //             for keycode in key_downs.iter() {
        //                 match keycode {
        //                     Keycode::D => {
        //                         set_input_dir(Direction::E);
        //                     }
        //                     Keycode::W => {
        //                         set_input_dir(Direction::N);
        //                     }
        //                     Keycode::S => {
        //                         set_input_dir(Direction::S);
        //                     }
        //                     Keycode::A => {
        //                         set_input_dir(Direction::W);
        //                     }
        //                     _ => {}
        //                 }
        //             }
        //         }
        //     }
        // }
    }

    fn set_rotational_input(&mut self, input: &WinitInputHelper) {
        fn set_input_turn(turn: Turn, world: &mut World) {
            // let mut query = <&mut RotationalInput>::query();
            // for input in query.iter_mut(world) {
            //     input.turn_sign = Some(turn);
            // }
            dev!("turning");
        }
        fn set_input_thrust(is_thrusting: bool, world: &mut World) {
            dev!("thrusting");
            // let mut query = <&mut RotationalInput>::query();
            // for input in query.iter_mut(world) {
            //     input.is_thrusting = is_thrusting;
            // }
        }

        if *self.loop_controller.get_state() == LoopState::Running {
            // HANDLE SINGLE MOVE KEYS
            if input.key_pressed(VirtualKeyCode::D) {
                set_input_turn(Turn::Right, &mut self.world);
            }
            // VirtualKeyCode::A => {
            //     set_input_turn(Turn::Left, &mut self.world);
            // }
            // VirtualKeyCode::W => {
            //     // set thrust
            //     set_input_thrust(true, &mut self.world);
            // }
            // VirtualKeyCode::S => {
            //     set_input_turn(Direction::S);
            // }
        }
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        // Logger::dbg("Drop game");
    }
}

fn rotate_point(x: f32, y: f32, rotation: f32, cx: f32, cy: f32) -> (f32, f32) {
    let x_translated = x - cx as f32;
    let y_translated = y - cy as f32;
    let x_rotated = x_translated * rotation.cos() + y_translated * rotation.sin();
    let y_rotated = x_translated * rotation.sin() - y_translated * rotation.cos();
    (x_rotated + cx as f32, y_rotated + cy as f32)
}

fn draw_ship(transform: &Transform, colorbody: &ColorBody, frame: &mut [u8]) {
    // TODO canonical length, to be used by draw and collision
    let r = 25.0;

    let x = transform.position.x;
    let y = transform.position.y;

    let mut x1 = x - r / 2.0;
    let mut y1 = y - r / 2.0;

    let mut x2 = x1;
    let mut y2 = y + r / 2.0;

    let mut x3 = x + r;
    let mut y3 = y;

    let mut xm = x + r / 20.0;
    let mut ym = y;

    let cx = (x1 + x2 + x3) / 3.0;
    let cy = (y1 + y2 + y3) / 3.0;

    (x1, y1) = rotate_point(x1, y1, transform.rotation, cx, cy);
    (xm, ym) = rotate_point(xm, ym, transform.rotation, cx, cy);
    (x2, y2) = rotate_point(x2, y2, transform.rotation, cx, cy);
    (x3, y3) = rotate_point(x3, y3, transform.rotation, cx, cy);

    // Draw the triangle
    // canvas
    //     .draw_line(
    //         Point::new(x1 as i32, y1 as i32),
    //         Point::new(xm as i32, ym as i32),
    //     )
    //     .unwrap();
    draw_line(
        x1.round() as i32,
        y1.round() as i32,
        xm.round() as i32,
        ym.round() as i32,
        colorbody.primary,
        frame,
    );

    // canvas
    //     .draw_line(
    //         Point::new(xm as i32, ym as i32),
    //         Point::new(x2 as i32, y2 as i32),
    //     )
    //     .unwrap();
    draw_line(
        xm.round() as i32,
        ym.round() as i32,
        x2.round() as i32,
        y2.round() as i32,
        colorbody.primary,
        frame,
    );
    // canvas
    //     .draw_line(
    //         Point::new(x2 as i32, y2 as i32),
    //         Point::new(x3 as i32, y3 as i32),
    //     )
    //     .unwrap();
    draw_line(
        x2.round() as i32,
        y2.round() as i32,
        x3.round() as i32,
        y3.round() as i32,
        colorbody.primary,
        frame,
    );
    // canvas
    //     .draw_line(
    //         Point::new(x3 as i32, y3 as i32),
    //         Point::new(x1 as i32, y1 as i32),
    //     )
    //     .unwrap();
    draw_line(
        x3.round() as i32,
        y3.round() as i32,
        x1.round() as i32,
        y1.round() as i32,
        colorbody.primary,
        frame,
    );
}
