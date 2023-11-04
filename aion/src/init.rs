use anyhow::Context;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::{
    gui::Framework, LOGICAL_WINDOW_HEIGHT, LOGICAL_WINDOW_WIDTH, PHYSICAL_WINDOW_HEIGHT,
    PHYSICAL_WINDOW_WIDTH, TITLE,
};

pub fn init_window(event_loop: &EventLoop<()>) -> Window {
    let logical_size = LogicalSize::new(LOGICAL_WINDOW_WIDTH, LOGICAL_WINDOW_HEIGHT);
    let physical_size = LogicalSize::new(PHYSICAL_WINDOW_WIDTH, PHYSICAL_WINDOW_HEIGHT);
    WindowBuilder::new()
        .with_title(TITLE)
        .with_inner_size(physical_size)
        .with_min_inner_size(logical_size)
        .build(&event_loop)
        .with_context(|| "Failed to create window")
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            panic!("Window initialization error");
        })
}

pub fn init_gfx(
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
