use anyhow::{Context, Result};
#[allow(warnings)]
use std::collections::HashMap;
use std::time::{Duration, Instant};
use winit::event_loop::EventLoop;

use winit::event::{Event, VirtualKeyCode};
use winit::window::Window;
use winit_input_helper::WinitInputHelper;

use crate::gui::Framework;
use crate::{log_error, WINDOW_HEIGHT, WINDOW_WIDTH}; // little function in main.rs
use legion::*;
use nalgebra_glm::Vec2;
use pixels::{Pixels, SurfaceTexture};

use super::systems::*;
use crate::components::*;
use crate::dsa::FixedSizeQueue;

const FRAMERATE: u8 = 60;
const FRAME_LIMIT_MS: f32 = 1000.0 / FRAMERATE as f32;

// Game Loop States
// eg:
// [init] => Stopped => [run] => Running => [input: pause] => Paused => [input: stop]
// => Stopped => [input: pause] => (no effect)Stopped => [input: start/resume] => Resuming => Running => [input: pause]
// => Paused => [input: unpause] => Running
pub enum RunState {
    Stopped, //  when render not running
    Running,
    Paused,
    Resuming, // transient state that marks going from Stopped -> Running
    Exiting,
}

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
    pub w: u32,
    pub h: u32,
}

#[derive(PartialEq, Eq, Hash)] // ? is Eq needed? what's it do?
enum ButtonState {
    Up,
    Down,
}

pub struct GameConfiguration;
pub struct Game {
    pub run_state: RunState,
    is_debug_on: bool,
    world: World,
    update_schedule: Schedule,
    resources: Resources,
    fps: f32,
    fps_queue: FixedSizeQueue,
    // key_states: HashMap<Keycode, Option<ButtonState>>,
}

impl Game {
    pub fn new() -> Result<Self, anyhow::Error> {
        env_logger::init();
        // Logger::dbg("INIT start");

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

        // Logger::dbg("INIT end");

        Ok(Self {
            run_state: RunState::Running,
            fps: 0.0,
            fps_queue: FixedSizeQueue::new(60),
            is_debug_on: false,
            world,
            update_schedule,
            resources,
        })
    }

    // fn handle_input(&mut self) {
    //     for event in self.event_pump.poll_iter() {
    //         match event {
    //             Event::Quit { .. }
    //             | Event::KeyDown {
    //                 keycode: Some(Keycode::Escape),
    //                 ..
    //             } => {
    //                 // if game already stopped, then quit, eg takes 2 ESCs to exit game
    //                 match self.run_state {
    //                     RunState::Stopped => {
    //                         Logger::info("Game exiting");
    //                         self.run_state = RunState::Exiting;
    //                     }
    //                     _ => {
    //                         Logger::info("Game stopped");
    //                         self.run_state = RunState::Stopped;
    //                     }
    //                 }
    //             }
    //             Event::KeyDown {
    //                 keycode: Some(key), ..
    //             } => {
    //                 self.key_states.insert(key, Some(ButtonState::Down));
    //             }
    //             Event::KeyUp {
    //                 keycode: Some(key), ..
    //             } => {
    //                 if let Some(buttonstate) = self.key_states.get_mut(&key) {
    //                     *buttonstate = Some(ButtonState::Up);
    //                 }
    //             }
    //             _ => {}
    //         }
    //     }
    //     self.handle_keys();
    //     // self.handle_key_up(keys_down);
    // }

    // TODO TimeContext
    pub fn handle_tick(&mut self, ms_prev_frame: &Instant) {
        let time_to_wait: f32 =
            FRAME_LIMIT_MS - Instant::now().duration_since(*ms_prev_frame).as_millis() as f32;

        // fixed frame rate: if below threshold MILLISECS_PER_FRAME then sleep
        if time_to_wait > 0.0 && time_to_wait <= FRAME_LIMIT_MS {
            let sleep_duration = Duration::new(0, (time_to_wait * 1000000.0) as u32);

            // * Sleeping for sleep_duration tends to actually sleep a little over by ~0.070 ms
            // let presleep = Instant::now();
            ::std::thread::sleep(sleep_duration);
            // let sleep_dur = Instant::now().duration_since(presleep);
            // println!("sleep_thread_duration(ms): {}", sleep_dur.as_micros() as f32 / 1000f32);

            // println!("ttw: {} sleep_for: {}", time_to_wait, sleep_duration.as_micros() as f64 /1000.0);
            if sleep_duration.as_millis() <= 2 {
                // Logger::info(&format!(
                // "Frames getting tight: sleeping {:?}ms",
                // sleep_duration.as_millis()
                // ));
            }
        }

        // aka elapsed seconds for a frame
        let dt = Dt(Instant::now().duration_since(*ms_prev_frame).as_secs_f32());
        self.resources.insert(dt.clone());
        // println!("dt_in_handletick: {}_sec", dt.0);

        self.fps_queue.push(1_f32 / dt.0); // dt to millis is u128
        self.fps = self.fps_queue.avg().unwrap_or(0f32);
    }

    pub fn setup(&mut self) {
        // Logger::dbg("SETUP start");

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
            CollisionArea { w: 50, h: 50 },
            // ColorBody {
            //     primary: Color::RGB(0, 255, 0),
            //     secondary: Color::RGB(0, 0, 0),
            // },
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
        //     (
        //         Transform {
        //             position: Vec2::new(500.0, 100.0),
        //             rotation: 0.0,
        //             scale: Vec2::new(1.0, 1.0),
        //         },
        //         RigidBody {
        //             velocity: Vec2::new(200.0, 100.0),
        //         },
        //         CollisionArea { w: 50, h: 50 },
        //         ColorBody {
        //             primary: Color::RGB(255, 255, 255),
        //             secondary: Color::RGB(0, 0, 0),
        //         },
        //     ),
        // ]);

        // Logger::dbg("SETUP end");
        self.resources.insert(Dt(0.01667f32));
    }

    // pub fn run(&mut self) {
    //     self.setup();
    //     self.run_state = RunState::Running;

    //     let mut ms_prev_frame = Instant::now();
    //     self.resources.insert(Dt(0.01667f32));
    //     Logger::dbg("Game loop running");
    //     loop {
    //         self.handle_tick(&ms_prev_frame);
    //         ms_prev_frame = Instant::now();
    //         self.handle_input();

    //         match self.run_state {
    //             RunState::Running => {
    //                 self.update();
    //             }
    //             RunState::Paused => {
    //                 // show pause menu
    //             }
    //             RunState::Stopped => {
    //                 continue;
    //             }
    //             RunState::Resuming => self.run_state = RunState::Running,
    //             RunState::Exiting => {
    //                 break;
    //             }
    //         }
    //         self.render();
    //     }
    // }

    pub fn update(&mut self) {
        self.update_schedule
            .execute(&mut self.world, &mut self.resources);
    }

    pub fn draw(&mut self, frame: &mut [u8]) {
        // Clear current rendering target with drawing color
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            pixel.copy_from_slice(&[0xFF, 0xFF, 0xFF, 0xff]);
        }
    }

    // pub fn render(&mut self) {
    //     let mut query = <(&Transform, &CollisionArea, &ColorBody)>::query();

    //     for (transform, _collision_area, colorbody) in query.iter(&self.world) {
    //         self.canvas.set_draw_color(colorbody.primary);
    //         draw_ship(transform, &mut self.canvas);
    //         // self.canvas
    //         //     .fill_rect(Rect::new(
    //         //         transform.position.x as i32,
    //         //         transform.position.y as i32,
    //         //         collision_area.w,
    //         //         collision_area.h,
    //         //     ))
    //         //     .expect("FAIL canvas.fill_rect");
    //     }
    //     self.canvas.present();
    // }

    pub fn destroy(&self) {
        // Logger::dbg("Destroy game");
    }

    // fn handle_keys(&mut self) {
    //     // HANDLE VALID SIMULTANEOUS MOVE KEYS
    //     let key_downs: Vec<Keycode> = self
    //         .key_states
    //         .iter()
    //         .filter_map(|(keycode, buttonstate)| {
    //             if *buttonstate == Some(ButtonState::Down) {
    //                 Some(*keycode)
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect();
    //     self.set_rotational_input(key_downs);
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

    // fn set_rotational_input(&mut self, key_downs: Vec<Keycode>) {
    //     fn set_input_turn(turn: Turn, world: &mut World) {
    //         let mut query = <&mut RotationalInput>::query();
    //         for input in query.iter_mut(world) {
    //             input.turn_sign = Some(turn);
    //         }
    //     }
    //     fn set_input_thrust(is_thrusting: bool, world: &mut World) {
    //         let mut query = <&mut RotationalInput>::query();
    //         for input in query.iter_mut(world) {
    //             input.is_thrusting = is_thrusting;
    //         }
    //     }

    //     if let RunState::Running = self.run_state {
    //         // ONLY ACTIVATE FOR TRANSLATIONAL HUMAN INPUTS... query the "player" input type
    //         // HANDLE SINGLE MOVE KEYS
    //         for keycode in key_downs.iter() {
    //             match keycode {
    //                 Keycode::D => {
    //                     set_input_turn(Turn::Right, &mut self.world);
    //                 }
    //                 Keycode::A => {
    //                     set_input_turn(Turn::Left, &mut self.world);
    //                 }
    //                 Keycode::W => {
    //                     // set thrust
    //                     set_input_thrust(true, &mut self.world);
    //                 }
    //                 // Keycode::S => {
    //                 //     set_input_turn(Direction::S);
    //                 // }
    //                 _ => {}
    //             }
    //         }
    //     }
    // }
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

// fn draw_ship(transform: &Transform, canvas: &mut Canvas<Window>) {
//     // TODO canonical length, to be used by draw and collision
//     let r = 25.0;

//     let x = transform.position.x;
//     let y = transform.position.y;

//     let mut x1 = x - r / 2.0;
//     let mut y1 = y - r / 2.0;

//     let mut x2 = x1;
//     let mut y2 = y + r / 2.0;

//     let mut x3 = x + r;
//     let mut y3 = y;

//     let mut xm = x + r / 20.0;
//     let mut ym = y;

//     let cx = (x1 + x2 + x3) / 3.0;
//     let cy = (y1 + y2 + y3) / 3.0;

//     (x1, y1) = rotate_point(x1, y1, transform.rotation, cx, cy);
//     (xm, ym) = rotate_point(xm, ym, transform.rotation, cx, cy);
//     (x2, y2) = rotate_point(x2, y2, transform.rotation, cx, cy);
//     (x3, y3) = rotate_point(x3, y3, transform.rotation, cx, cy);

//     // Draw the triangle
//     canvas
//         .draw_line(
//             Point::new(x1 as i32, y1 as i32),
//             Point::new(xm as i32, ym as i32),
//         )
//         .unwrap();
//     canvas
//         .draw_line(
//             Point::new(xm as i32, ym as i32),
//             Point::new(x2 as i32, y2 as i32),
//         )
//         .unwrap();
//     canvas
//         .draw_line(
//             Point::new(x2 as i32, y2 as i32),
//             Point::new(x3 as i32, y3 as i32),
//         )
//         .unwrap();
//     canvas
//         .draw_line(
//             Point::new(x3 as i32, y3 as i32),
//             Point::new(x1 as i32, y1 as i32),
//         )
//         .unwrap();
// }

/// Representation of the application state. In this example, a box will bounce around the screen.
const WIDTH: usize = 320;
const HEIGHT: usize = 240;

struct Box {
    box_x: usize,
    box_y: usize,
    w: usize,
    h: usize,
    velocity_x: i16,
    velocity_y: i16,
}

impl Box {
    fn new() -> Self {
        Box {
            box_x: 24,
            box_y: 16,
            velocity_x: 5,
            velocity_y: 5,
            w: 25,
            h: 25,
        }
    }
    fn update(&mut self) {
        self.box_x = (self.box_x as i16 + self.velocity_x) as usize;
        self.box_y = (self.box_y as i16 + self.velocity_y) as usize;

        if self.box_x <= 0 || self.box_x + self.w > WIDTH {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + self.h > HEIGHT {
            self.velocity_y *= -1;
        }

        if self.box_x + self.w > WIDTH {
            self.box_x = WIDTH - self.w;
        }
        if self.box_y + self.h > HEIGHT {
            self.box_y = HEIGHT - self.h;
        }
    }
    fn draw(&self, frame: &mut [u8]) {
        for y in self.box_y..self.box_y + self.h - 1 {
            for x in self.box_x..self.box_x + self.w - 1 {
                let i = (y * WIDTH + x) * 4;
                let color = [0x5e, 0x48, 0xe8, 0xff];
                frame[i..i + 4].copy_from_slice(&color);
            }
        }
    }
}
