#![allow(unused)]
mod archetypes;
mod audio;
mod avatars;
mod components;
mod game;
mod gui;
mod init;
mod scenario;
mod systems;

pub mod gfx;
pub mod util;

extern crate procfs;

use std::{
    cell::{RefCell, RefMut},
    env,
    rc::Rc,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use gfx::pixel::{Color, BLACK};
use gui::Framework;
use pixels::{Error, Pixels, SurfaceTexture};
use util::monitor::get_process_memory;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use winit_input_helper::WinitInputHelper;

use gui::StateMonitor;
use init::{init_gfx, init_window};
use util::logging::log_error;
use util::time::FrameTimer;

use audio_manager::SoundManager;
use game::Game;
use game::{GetRunState, RunState};
use game_loop::game_loop;

fn process_dbg_keys(game: &mut Game, dbg_ctx: &mut DebugContext) {
    if game.input.key_pressed(VirtualKeyCode::P) {
        if game.get_runstate() == RunState::Running {
            game.loop_controller.pause();
        } else if game.get_runstate() == RunState::Paused {
            game.loop_controller.run();
        }
    }
    if game.input.key_pressed(VirtualKeyCode::Semicolon) {
        if game.get_runstate() == RunState::Stopped {
            game.loop_controller.run();
        } else if game.get_runstate() != RunState::Stopped {
            game.loop_controller.stop();
        }
    }
    if game.input.key_pressed(VirtualKeyCode::Grave) {
        dbg_ctx.is_on = !dbg_ctx.is_on;
    }
    if game.input.key_pressed(VirtualKeyCode::Key1) {
        dbg_ctx.is_drawing_collisionareas = !dbg_ctx.is_drawing_collisionareas;
    }
}

pub struct DebugContext {
    is_on: bool,
    is_drawing_collisionareas: bool,
}

struct RenderContext {
    pixels: Rc<RefCell<Pixels>>,
    framework: Rc<RefCell<Framework>>,
    render_timer: FrameTimer,
    update_timer: Rc<RefCell<FrameTimer>>,
}

struct UpdateContext {
    update_timer: Rc<RefCell<FrameTimer>>,
}

struct InputContext {
    pixels: Rc<RefCell<Pixels>>,
    framework: Rc<RefCell<Framework>>,
}

pub static LOGICAL_WINDOW_WIDTH: f32 = 960.;
pub static LOGICAL_WINDOW_HEIGHT: f32 = 540.;
pub static PHYSICAL_WINDOW_WIDTH: f32 = 1920.;
pub static PHYSICAL_WINDOW_HEIGHT: f32 = 1080.;
const TITLE: &'static str = "Aion";
const UPDATES_PER_SECOND: u32 = 60;
const MAX_FRAME_TIME: f64 = 0.1;

fn main() {
    env::set_var("RUST_LOG", "DEV=debug");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .target(env_logger::Target::Stdout)
        .init();

    let event_loop = EventLoop::new();
    let window = init_window(&event_loop);

    let window = Arc::new(window);

    let (mut pixels, mut framework) = init_gfx(&event_loop, &window).unwrap();

    // data for update closure
    let pixels_render = Rc::new(RefCell::new(pixels));
    let pixels_input = Rc::clone(&pixels_render);

    let framework_render = Rc::new(RefCell::new(framework));
    let framework_input = Rc::clone(&framework_render);

    let mut update_timer = Rc::new(RefCell::new(FrameTimer::new()));
    let mut update_timer_render = Rc::clone(&update_timer);

    let mut render_timer = FrameTimer::new();

    // data for update closure
    let mut update_ctx = Box::new(UpdateContext { update_timer });

    // data for render closure
    let mut render_ctx = Box::new(RenderContext {
        pixels: pixels_render,
        framework: framework_render,
        render_timer: render_timer,
        update_timer: update_timer_render,
    });

    // data for input closure
    let mut input_ctx = Box::new(InputContext {
        pixels: pixels_input,
        framework: framework_input,
    });

    let mut dbg_ctx = DebugContext {
        is_on: false,
        is_drawing_collisionareas: false,
    };

    let dbg_ctx = Rc::new(RefCell::new(dbg_ctx));
    let dbg_ctx_render = Rc::clone(&dbg_ctx);
    let dbg_ctx_input = Rc::clone(&dbg_ctx);
    let dbg_ctx_gui = Rc::clone(&dbg_ctx);

    let mut memstat: Option<u64> = None;

    let mut game = Game::new().unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });

    game.setup();

    game_loop(
        event_loop,
        window,
        game,
        UPDATES_PER_SECOND,
        MAX_FRAME_TIME,
        move |g| {
            if g.game.get_runstate() == RunState::Running {
                let mut update_timer1 = update_ctx.update_timer.borrow_mut();
                let dt = update_timer1.tick();
                g.game.update(dt);
            }
        },
        move |g| {
            if g.game.get_runstate() != RunState::Stopped {
                let rdt = render_ctx.render_timer.tick();

                ////////////////////////////////////////////////////////////////////
                // RENDER
                ////////////////////////////////////////////////////////////////////

                let mut framework = render_ctx.framework.borrow_mut();
                let mut pixels = render_ctx.pixels.borrow_mut();

                g.game.render(&mut pixels, &dbg_ctx_render.borrow(), rdt);

                let render_timer = &render_ctx.render_timer;
                let update_timer2 = render_ctx.update_timer.borrow();

                if render_timer.count_frames() % 60 == 0 {
                    memstat = get_process_memory();
                }

                let gui_game_state = StateMonitor {
                    game: &mut g.game,
                    render_fps: render_timer.fps(),
                    update_fps: update_timer2.fps(),
                    render_frame_count: render_timer.count_frames(),
                    update_frame_count: update_timer2.count_frames(),
                    dbg_ctx: &mut dbg_ctx_gui.borrow_mut(),
                    memstat,
                };

                framework.prepare(&g.window, gui_game_state); // Prepare egui

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);
                    framework.render(encoder, render_target, context); // Render egui
                    Ok(())
                });

                if let Err(err) = render_result {
                    log_error("pixels.render", err);
                    g.exit();
                }
            }
        },
        ////////////////////////////////////////////////////////////////////
        // INPUT
        ////////////////////////////////////////////////////////////////////
        // move not in original tuzz code
        move |g, event| {
            let mut framework = input_ctx.framework.borrow_mut();
            let mut pixels = input_ctx.pixels.borrow_mut();

            match event {
                Event::WindowEvent { event, .. } => {
                    framework.handle_event(event);
                }
                _ => {}
            }

            if g.game.input.update(event) {
                process_dbg_keys(&mut g.game, &mut dbg_ctx_input.borrow_mut());

                if g.game.input.close_requested()
                    || g.game.input.key_pressed(VirtualKeyCode::Escape)
                {
                    g.game.loop_controller.exit();
                    g.exit();
                    return;
                }

                if let Some(scale_factor) = g.game.input.scale_factor() {
                    framework.scale_factor(scale_factor);
                }

                if let Some(size) = g.game.input.window_resized() {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        log_error("pixels.resize_surface", err);
                        g.game.loop_controller.exit();
                        g.exit()
                    }
                    framework.resize(size.width, size.height);
                }
            }
        },
    );
}
