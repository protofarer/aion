/// The Context
/// - track hardware states
///     - screen
///     - audio
///     - timers
/// - one per game

pub struct Context {
    // pub gfx: GraphicsContext,
    // pub time: timer::TimeContext,
    // pub audio: AudioContext,
    // pub keyboard: input::keyboard::KeyboardContext,
    // pub continuing: bool // event loop control run
    pub quit_requested: bool, // from event loop
}

impl Context {
    pub fn new() -> (Context, winit::event_loop::EventLoop<()>) {
        let event_loop = winit::event_loop::EventLoop::new();
        let ctx = Context {
            quit_requested: false,
        };
        (ctx, event_loop)
    }
    pub fn request_quit(&mut self) {
        self.quit_requested = true;
    }
}
