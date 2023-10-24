use std::collections::VecDeque;
use std::iter::Sum;
use std::time;

const N_FRAME_LOGS: usize = 200;

pub struct Dt(pub time::Duration);
impl Dt {
    pub fn secs(&self) -> f32 {
        self.0.as_secs_f32()
    }
    pub fn micros(&self) -> u128 {
        self.0.as_micros()
    }
}

#[derive(Debug, Clone)]
pub struct FrameLogger<T>
where
    T: Clone,
{
    data: VecDeque<T>,
    capacity: usize,
}

impl<T> FrameLogger<T>
where
    T: Copy,
{
    pub fn new(capacity: usize, init_val: T) -> FrameLogger<T> {
        let mut vecdeq = VecDeque::with_capacity(capacity);
        vecdeq.push_front(init_val);
        FrameLogger {
            data: vecdeq,
            capacity,
        }
    }
    pub fn push(&mut self, value: T) {
        if self.data.len() >= self.capacity {
            self.data.pop_front();
        }
        self.data.push_back(value);
    }
    pub fn contents(&self) -> &VecDeque<T> {
        &self.data
    }
    pub fn last(&self) -> T {
        self.data[self.data.len() - 1]
    }
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

pub struct FrameTimer {
    last_instant: time::Instant,
    init_instant: time::Instant,
    frame_log: FrameLogger<time::Duration>,
    frame_count: usize,
}

impl FrameTimer {
    pub fn new() -> FrameTimer {
        let init_dt = time::Duration::from_millis(16);
        FrameTimer {
            init_instant: time::Instant::now(),
            last_instant: time::Instant::now(),
            frame_log: FrameLogger::new(N_FRAME_LOGS, init_dt),
            frame_count: 0,
        }
    }
    pub fn avg_dt(&self) -> time::Duration {
        let sum: time::Duration = self.frame_log.contents().iter().sum();
        sum / u32::try_from(self.frame_log.contents().len()).unwrap()
    }
    pub fn fps(&self) -> f64 {
        let avg_dt = self.avg_dt();
        (1.0 / avg_dt.as_secs_f64()).round()
    }
    pub fn count_frames(&self) -> usize {
        self.frame_count
    }
    pub fn time_since_start(&self) -> time::Duration {
        self.init_instant.elapsed()
    }
    pub fn clear_log(&mut self) {
        self.frame_log.clear();
    }
    pub fn reset(&mut self) {
        self.clear_log();
        self.frame_count = 0;
    }
    pub fn tick(&mut self) -> Dt {
        let dt = self.last_instant.elapsed();
        self.frame_log.push(dt);
        self.last_instant = time::Instant::now();
        self.frame_count += 1;
        Dt(dt)
    }
}

impl Default for FrameTimer {
    fn default() -> Self {
        Self::new()
    }
}

pub struct GameLoop {
    update_count: u32,
    render_count: u32,
    running_time: f64,
    accumulated_time: f64,
}
