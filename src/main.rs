#![allow(unused)]
mod components;
mod draw;
mod game;
mod geom;
mod gui;
mod pixel;
mod systems;
mod time;

use std::{env, sync::Arc, time::Instant};

use crate::game::Game;
use error_iter::ErrorIter as _;
use game::{Dt, GetLoopState, LoopState};
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

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    error!("{method_name}() failed: {err}");
    for source in err.sources().skip(1) {
        error!("  Caused by: {source}");
    }
}

fn init_window(event_loop: &EventLoop<()>) -> Window {
    let size = winit::dpi::LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    winit::window::WindowBuilder::new()
        .with_title("my_window")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap()
}

fn init_gfx(
    event_loop: &EventLoop<()>,
    window: &Window,
) -> Result<(Pixels, Framework), pixels::Error> {
    let (pixels, framework) = {
        let scale_factor = window.scale_factor() as f32;
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32, surface_texture)?;
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

fn run(
    event_loop: EventLoop<()>,
    window: Window,
    mut pixels: Pixels,
    mut framework: Framework,
    mut input: WinitInputHelper,
    mut ctx: Context,
    mut game: Game,
) -> Result<(), Error> {
    game.setup();
    game.loop_controller.run();
    let mut timer = FrameTimer::new(16);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        // Handle input events
        if input.update(&event) {
            if input.close_requested()
                || (*game.get_loopstate() == LoopState::Stopped
                    && input.key_pressed(VirtualKeyCode::Escape))
            {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                framework.resize(size.width, size.height);
            }

            game.process_input();

            if *game.get_loopstate() == LoopState::Exiting {
                *control_flow = ControlFlow::Exit;
            }
        }

        match event {
            Event::MainEventsCleared => {
                ////////////////////////////////////////////////////////////////////
                // UPDATE
                ////////////////////////////////////////////////////////////////////
                let _dt = timer.tick();
                println!("dt: {}", _dt.as_millis());
                // if *game.get_loopstate() == LoopState::Running {
                //     game.update();
                // }
                ////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////
                window.request_redraw();
            }
            Event::WindowEvent { event, .. } => {
                framework.handle_event(&event);
            }
            Event::RedrawRequested(_) => {
                ////////////////////////////////////////////////////////////////////
                // RENDER
                ////////////////////////////////////////////////////////////////////
                // Clear current rendering target with drawing color
                // a faster clear?
                for (i, pixel) in pixels.frame_mut().chunks_exact_mut(4).enumerate() {
                    pixel.copy_from_slice(BLACK.as_bytes());
                }

                // Mutate frame buffer
                // game.draw(pixels.frame_mut());

                // Prepare egui
                // framework.prepare(&window);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    // framework.render(encoder, render_target, context);

                    Ok(())
                });

                if let Err(err) = render_result {
                    log_error("pixels.render", err);
                    *control_flow = ControlFlow::Exit;
                }
                ////////////////////////////////////////////////////////////////////
                ////////////////////////////////////////////////////////////////////
            }
            _ => (),
        }
    });
}

struct Context {
    is_debug_on: bool,
}
impl Context {
    pub fn new() -> Self {
        Context { is_debug_on: false }
    }
}

#[macro_export]
macro_rules! dev {
    ($($arg:tt)*) => {
        log::debug!(target: "DEV", $($arg)*);
    }
}

fn main() {
    env::set_var("RUST_LOG", "DEV=debug");
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
        .target(env_logger::Target::Stdout)
        .init();

    let event_loop = EventLoop::new();
    let window = init_window(&event_loop);

    let (mut pixels, mut framework) = init_gfx(&event_loop, &window).unwrap();
    let mut input = WinitInputHelper::new();

    let mut ctx = Context::new();
    let mut game = Game::new(pixels).unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });
    // run(event_loop, window, pixels, framework, input, ctx, game);

    game.setup();
    game.loop_controller.run();
    let window = Arc::new(window);
    let mut timer = FrameTimer::new(60);

    game_loop(
        event_loop,
        window,
        game,
        10 as u32,
        0.5,
        move |g| {
            // Update the world
            g.game.update();
        },
        move |g| {
            let dt = timer.tick();
            println!("fps: {}", timer.fps().round());
            // Drawing
            ////////////////////////////////////////////////////////////////////
            // RENDER
            ////////////////////////////////////////////////////////////////////
            // Clear current rendering target with drawing color
            // a faster clear?
            let mut frame = g.game.pixels.frame_mut();
            for pixel in frame.chunks_exact_mut(4) {
                pixel.copy_from_slice(BLACK.as_bytes());
            }
            // g.game.draw();

            // Mutate frame buffer
            // game.draw(pixels.frame_mut());

            // Prepare egui
            // framework.prepare(&window);

            // Render everything together
            let render_result = g
                .game
                .pixels
                .render_with(|encoder, render_target, context| {
                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);

                    // Render egui
                    // framework.render(encoder, render_target, context);

                    Ok(())
                });

            if let Err(err) = render_result {
                log_error("pixels.render", err);
                g.exit();
                // *control_flow = ControlFlow::Exit;
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
        |g, event| {
            // Let winit_input_helper collect events to build its state.
            if g.game.input.update(event) {
                // Update controls
                g.game.process_input();

                // Close events
                if g.game.input.key_pressed(VirtualKeyCode::Escape)
                    || g.game.input.close_requested()
                {
                    g.game.loop_controller.exit();
                    g.exit();
                    return;
                }
                // Resize the window
                if let Some(size) = g.game.input.window_resized() {
                    if let Err(err) = g.game.pixels.resize_surface(size.width, size.height) {
                        log_error("pixels.resize_surface", err);
                        g.game.loop_controller.exit();
                    }
                }
            }
        },
    );
}
