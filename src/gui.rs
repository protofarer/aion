use std::{cell::RefCell, rc::Rc};

use egui::{ClippedPrimitive, Context, Pos2, TexturesDelta};
use egui_wgpu::{
    renderer::{Renderer, ScreenDescriptor},
    wgpu::Device,
};
use pixels::{wgpu, PixelsContext};
use winit::event_loop::EventLoopWindowTarget;
use winit::window::Window;

use crate::{
    archetypes::{
        gen_buncha_rng_circloids, gen_buncha_rng_particles, gen_buncha_rng_projectiles,
        gen_circloids,
    },
    dev,
    game::{Game, GetRunState, RunState},
    DebugContext, LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH, PHYSICAL_WINDOW_HEIGHT,
    PHYSICAL_WINDOW_WIDTH,
};

const EGUI_RED: egui::Color32 = egui::Color32::from_rgb(255, 0, 0);
const EGUI_GREEN: egui::Color32 = egui::Color32::from_rgb(0, 255, 0);
const EGUI_BLUE: egui::Color32 = egui::Color32::from_rgb(0, 0, 255);
const EGUI_WHITE: egui::Color32 = egui::Color32::from_rgb(255, 255, 255);
const EGUI_BLACK: egui::Color32 = egui::Color32::from_rgb(0, 0, 0);
const EGUI_ORANGE: egui::Color32 = egui::Color32::from_rgb(255, 165, 0);
const EGUI_YELLOW: egui::Color32 = egui::Color32::from_rgb(255, 255, 0);
const EGUI_MAGENTA: egui::Color32 = egui::Color32::from_rgb(255, 0, 255);

/// Manages all state required for rendering egui over `Pixels`.
pub struct Framework {
    // State for egui.
    egui_ctx: Context,
    egui_state: egui_winit::State,
    screen_descriptor: ScreenDescriptor,
    renderer: Renderer,
    paint_jobs: Vec<ClippedPrimitive>,
    textures: TexturesDelta,

    // State for the GUI
    gui: Gui,
}

/// Example application state. A real application will need a lot more state than this.
struct Gui {
    /// Only show the egui window when true.
    window_open: bool,
    n_spawn_circloids: i32,
    n_spawn_particles: i32,
    n_spawn_projectiles: i32,
}

impl Framework {
    /// Create egui.
    pub(crate) fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        width: u32,
        height: u32,
        scale_factor: f32,
        pixels: &pixels::Pixels,
    ) -> Self {
        let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        let mut egui_state = egui_winit::State::new(event_loop);
        egui_state.set_max_texture_side(max_texture_size);
        egui_state.set_pixels_per_point(scale_factor);
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: scale_factor,
        };
        let renderer = Renderer::new(pixels.device(), pixels.render_texture_format(), None, 1);
        let textures = TexturesDelta::default();
        let gui = Gui::new();

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            renderer,
            paint_jobs: Vec::new(),
            textures,
            gui,
        }
    }

    /// Handle input events from the window manager.
    pub(crate) fn handle_event(&mut self, event: &winit::event::WindowEvent) {
        let _ = self.egui_state.on_event(&self.egui_ctx, event);
    }

    /// Resize egui.
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.size_in_pixels = [width, height];
        }
    }

    /// Update scaling factor.
    pub(crate) fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.pixels_per_point = scale_factor as f32;
    }

    /// Prepare egui.
    pub(crate) fn prepare(&mut self, window: &Window, game_state: StateMonitor) {
        // Run the egui frame and create all paint jobs to prepare for rendering.
        let raw_input = self.egui_state.take_egui_input(window);
        let output = self.egui_ctx.run(raw_input, |egui_ctx| {
            ////////////////////////////////////////////////////////////////////
            // Draw the demo application.
            ////////////////////////////////////////////////////////////////////
            self.gui.ui(egui_ctx, game_state);
            ////////////////////////////////////////////////////////////////////
            ////////////////////////////////////////////////////////////////////
        });

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(window, &self.egui_ctx, output.platform_output);
        self.paint_jobs = self.egui_ctx.tessellate(output.shapes);
    }

    /// Render egui.
    pub(crate) fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) {
        // Upload all resources to the GPU.
        for (id, image_delta) in &self.textures.set {
            self.renderer
                .update_texture(&context.device, &context.queue, *id, image_delta);
        }
        self.renderer.update_buffers(
            &context.device,
            &context.queue,
            encoder,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Render egui with WGPU
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: render_target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.renderer
                .render(&mut rpass, &self.paint_jobs, &self.screen_descriptor);
        }

        // Cleanup
        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.renderer.free_texture(id);
        }
    }
}

impl Gui {
    /// Create a `Gui`.
    fn new() -> Self {
        Self {
            window_open: true,
            n_spawn_particles: 1,
            n_spawn_circloids: 1,
            n_spawn_projectiles: 1,
        }
    }

    /// Create the UI using egui.
    fn ui(&mut self, ctx: &Context, gs: StateMonitor) {
        let run_state = match gs.game.get_runstate() {
            RunState::Running => "running",
            RunState::Exiting => "exiting",
            RunState::Paused => "paused",
            RunState::Stopped => "stopped",
        };

        // MENU BAR
        egui::TopBottomPanel::top("menubar_container").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("About...").clicked() {
                        self.window_open = true;
                        ui.close_menu();
                    }
                    if ui.button("Exit").clicked() {
                        self.window_open = false;
                        ui.close_menu();
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "mem_rss: {} MB",
                        ((gs.memstat.unwrap_or(0u64) as f64 / 1_000_000.) * 10.).floor() / 10.0
                    ));
                });
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "win dims: {}x{} ",
                        PHYSICAL_WINDOW_WIDTH, PHYSICAL_WINDOW_HEIGHT
                    ));
                    ui.label(run_state);
                    ui.label(
                        egui::RichText::new(format!(
                            "render[ fps: {} cnt: {} ]",
                            gs.render_fps, gs.render_frame_count
                        ))
                        .color(match gs.render_fps as i32 {
                            0..=29 => EGUI_RED,
                            30..=59 => EGUI_ORANGE,
                            60 => EGUI_GREEN,
                            _ => EGUI_WHITE,
                        }),
                    );
                    ui.label(format!(
                        "update[ fps: {} cnt: {} ]",
                        gs.update_fps, gs.update_frame_count
                    ));
                    ui.label(format!(" n_ents: {}", gs.game.world.len()));
                })
            });
        });

        // WINDOW: DEBUG INFO AND GAME STATE MUTATION
        if gs.dbg_ctx.is_on {
            egui::Window::new("Debug Display")
                .open(&mut self.window_open)
                .default_pos(Pos2::new(PHYSICAL_WINDOW_WIDTH, 0.0))
                .show(ctx, |ui| {
                    ui.checkbox(
                        &mut gs.dbg_ctx.is_drawing_collisionareas,
                        "show collision areas",
                    );

                    ui.horizontal(|ui| {
                        if ui.button("spawn particles").clicked() {
                            gs.game
                                .world
                                .extend(gen_buncha_rng_particles(self.n_spawn_particles));
                        }
                        ui.add(egui::Slider::new(&mut self.n_spawn_particles, 1..=10).step_by(1.));
                    });
                    ui.horizontal(|ui| {
                        if ui.button("spawn circloids").clicked() {
                            gs.game
                                .world
                                .extend(gen_buncha_rng_circloids(self.n_spawn_circloids));
                        }
                        ui.add(egui::Slider::new(&mut self.n_spawn_circloids, 1..=10).step_by(1.));
                    });
                    ui.horizontal(|ui| {
                        if ui.button("spawn projectiles").clicked() {
                            gs.game
                                .world
                                .extend(gen_buncha_rng_projectiles(self.n_spawn_projectiles));
                        }
                        ui.add(
                            egui::Slider::new(&mut self.n_spawn_projectiles, 1..=10).step_by(1.),
                        );
                    });

                    if ui.button("step update").clicked() {
                        dev!("step update");
                    }

                    if ui.button("step render").clicked() {
                        dev!("step render");
                    }

                    if ui.button("restart").clicked() {
                        dev!("step render");
                    }

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x /= 2.0;
                        ui.label("Learn more about egui at");
                    });
                });
        }
    }
}

// #[derive(Copy, Clone)]
pub struct StateMonitor<'a> {
    pub game: &'a mut Game,
    pub render_fps: f64,
    pub update_fps: f64,
    pub render_frame_count: usize,
    pub update_frame_count: usize,
    pub dbg_ctx: &'a mut DebugContext,
    pub memstat: Option<u64>,
}
