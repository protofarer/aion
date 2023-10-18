#![allow(unused)]
mod components;
mod dsa;
mod game;
mod gui;
mod systems;

use std::{env, time::Instant};

use crate::game::Game;
use error_iter::ErrorIter as _;
use game::{Dt, GetLoopState, LoopState};
use gui::Framework;
use log::{error, info};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use winit_input_helper::WinitInputHelper;

pub const WINDOW_WIDTH: u32 = 1280;
pub const WINDOW_HEIGHT: u32 = 720;

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
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture)?;
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
    let mut ms_prev_frame = Instant::now();
    game.loop_controller.run();
    let mut boxent = Box::new();

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();
        // TimeContext
        // game.handle_tick(&ms_prev_frame);
        // ms_prev_frame = Instant::now();

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

            game.process_input(&input);

            match *game.get_loopstate() {
                LoopState::Exiting => {
                    *control_flow = ControlFlow::Exit;
                }
                LoopState::Running => {
                    // game.update();
                    boxent.update();
                    window.request_redraw();
                }
                LoopState::Paused => {
                    window.request_redraw();
                }
                _ => {}
            }
        }

        // RENDER
        match event {
            Event::WindowEvent { event, .. } => {
                framework.handle_event(&event);
            }
            Event::RedrawRequested(_) => {
                // Fill frame buffer
                game.draw(pixels.frame_mut());
                boxent.draw(pixels.frame_mut());

                // Prepare egui
                framework.prepare(&window);

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
                    *control_flow = ControlFlow::Exit;
                }
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
    let mut game = Game::new().unwrap_or_else(|e| {
        println!("{e}");
        std::process::exit(1);
    });
    run(event_loop, window, pixels, framework, input, ctx, game);
}

struct Box {
    box_x: i32,
    box_y: i32,
    w: i32,
    h: i32,
    velocity_x: i32,
    velocity_y: i32,
}

impl Box {
    fn new() -> Self {
        Box {
            box_x: 25,
            box_y: 15,
            velocity_x: 15,
            velocity_y: 15,
            w: 25,
            h: 25,
        }
    }
    fn update(&mut self) {
        self.box_x = (self.box_x + self.velocity_x);
        self.box_y = (self.box_y + self.velocity_y);

        if self.box_x <= 0 || self.box_x + self.w > WINDOW_WIDTH as i32 {
            self.velocity_x *= -1;
        }
        if self.box_y <= 0 || self.box_y + self.h > WINDOW_HEIGHT as i32 {
            self.velocity_y *= -1;
        }
        if self.box_y < 0 {
            self.box_y = 0;
        }
        if self.box_x < 0 {
            self.box_x = 0;
        }
        if self.box_x + self.w > WINDOW_WIDTH as i32 {
            self.box_x = WINDOW_WIDTH as i32 - self.w;
        }
        if self.box_y + self.h > WINDOW_HEIGHT as i32 {
            self.box_y = WINDOW_HEIGHT as i32 - self.h;
        }
    }
    fn draw(&self, frame: &mut [u8]) {
        for y in self.box_y..self.box_y + self.h {
            for x in self.box_x..self.box_x + self.w {
                let i: usize = ((y * WINDOW_WIDTH as i32 + x) * 4) as usize;
                let color = [0x5e, 0x48, 0xe8, 0xff];
                frame[i..i + 4].copy_from_slice(&color);
            }
        }
    }
}
