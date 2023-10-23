#![allow(unused)]
mod components;
mod draw;
mod draw_bodies;
mod game;
mod geom;
mod gui;
mod pixel;
mod systems;
mod time;

use std::{
    cell::RefCell,
    env,
    rc::Rc,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use crate::{game::Game, gui::StateMonitor};
use error_iter::ErrorIter as _;
use game::{GetRunState, RunState};
use game_loop::game_loop;
use gui::Framework;
use log::{error, info};
use pixel::{Color, BLACK};
use pixels::{Error, Pixels, SurfaceTexture};
use time::FrameTimer;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use winit_input_helper::WinitInputHelper;

pub const TITLE: &str = "Aion";
pub const LOGICAL_WINDOW_WIDTH: f32 = 960.;
pub const LOGICAL_WINDOW_HEIGHT: f32 = 540.;
pub const PHYSICAL_WINDOW_WIDTH: f32 = 1920.;
pub const PHYSICAL_WINDOW_HEIGHT: f32 = 1080.;
pub const INIT_DT: Duration = Duration::from_millis(16);
const UPDATES_PER_SECOND: u32 = 120;
const MAX_FRAME_TIME: f64 = 0.1;

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

fn init_window(event_loop: &EventLoop<()>) -> Window {
    let logical_size = winit::dpi::LogicalSize::new(LOGICAL_WINDOW_WIDTH, LOGICAL_WINDOW_HEIGHT);
    let physical_size = LogicalSize::new(PHYSICAL_WINDOW_WIDTH, PHYSICAL_WINDOW_HEIGHT);
    winit::window::WindowBuilder::new()
        .with_title(TITLE)
        .with_inner_size(physical_size)
        .with_min_inner_size(logical_size)
        .build(&event_loop)
        .unwrap()
}

fn init_gfx(
    event_loop: &EventLoop<()>,
    window: &Window,
) -> Result<(Pixels, Framework), pixels::Error> {
    let (pixels, framework) = {
        let scale_factor = window.scale_factor() as f32;
        let window_size = window.inner_size(); // Physical screen dims (scaled from logical)
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        let pixels = Pixels::new(
            LOGICAL_WINDOW_WIDTH as u32,
            LOGICAL_WINDOW_HEIGHT as u32,
            surface_texture,
        )?;

        let framework = Framework::new(
            event_loop,
            window_size.width,
            window_size.height,
            scale_factor,
            &pixels,
        );

        (pixels, framework)
    };

    Ok((pixels, framework))
}

#[macro_export]
macro_rules! dev {
    ($($arg:tt)*) => {
        log::debug!(target: "DEV", $($arg)*);
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

fn main() {
    env::set_var("RUST_LOG", "DEV=debug");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .target(env_logger::Target::Stdout)
        .init();

    let event_loop = EventLoop::new();
    let window = init_window(&event_loop);
    let window = Arc::new(window);
    // let mut input = WinitInputHelper::new();

    let (mut pixels, mut framework) = init_gfx(&event_loop, &window).unwrap();

    // data for update closure
    let pixels1 = Rc::new(RefCell::new(pixels));
    let pixels2 = Rc::clone(&pixels1);

    let framework1 = Rc::new(RefCell::new(framework));
    let framework2 = Rc::clone(&framework1);

    let mut update_timer1 = Rc::new(RefCell::new(FrameTimer::new()));
    let mut update_timer2 = Rc::clone(&update_timer1);

    let mut render_timer = FrameTimer::new();

    // data for update closure
    let mut update_ctx = Box::new(UpdateContext {
        update_timer: update_timer1,
    });

    // data for render closure
    let mut render_ctx = Box::new(RenderContext {
        pixels: pixels1,
        framework: framework1,
        render_timer: render_timer,
        update_timer: update_timer2,
    });

    // data for input closure
    let mut input_ctx = Box::new(InputContext {
        pixels: pixels2,
        framework: framework2,
    });

    let mut dbg_ctx = DebugContext {
        is_on: false,
        is_drawing_collisionareas: false,
    };

    let dbg_ctx = Rc::new(RefCell::new(dbg_ctx));
    let dbg_ctx_render = Rc::clone(&dbg_ctx);
    let dbg_ctx_input = Rc::clone(&dbg_ctx);
    let dbg_ctx_gui = Rc::clone(&dbg_ctx);

    let mut game = Game::new().unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });

    game.setup();
    game.loop_controller.run();

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
                let _ = render_ctx.render_timer.tick();

                ////////////////////////////////////////////////////////////////////
                // RENDER
                ////////////////////////////////////////////////////////////////////
                ///
                let mut framework = render_ctx.framework.borrow_mut();
                let mut pixels = render_ctx.pixels.borrow_mut();

                g.game.render(&mut pixels, &dbg_ctx_render.borrow());

                // Prepare egui

                let render_timer = &render_ctx.render_timer;
                let update_timer2 = render_ctx.update_timer.borrow();

                let gui_game_state = StateMonitor {
                    game: &mut g.game,
                    render_fps: render_timer.fps(),
                    update_fps: update_timer2.fps(),
                    render_frame_count: render_timer.count_frames(),
                    update_frame_count: update_timer2.count_frames(),
                    dbg_ctx: &mut dbg_ctx_gui.borrow_mut(),
                };

                framework.prepare(&g.window, gui_game_state);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    framework.render(encoder, render_target, context);

                    Ok(())
                });

                if let Err(err) = render_result {
                    log_error("pixels.render", err);
                    g.exit();
                }
            }
            ////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////

            // Sleep the main thread to limit drawing to the fixed time step.
            // See: https://github.com/parasyte/pixels/issues/174

            // let dt = TIME_STEP.as_secs_f64() - Time::now().sub(&g.current_instant());
            // if dt > 0.0 {
            //     std::thread::sleep(Duration::from_secs_f64(dt));
            // }
        },
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
                g.game.process_input(&mut dbg_ctx_input.borrow_mut());

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
