use std::collections::VecDeque;

// TODO use traits to cover any from u8 to u64 depending on limit
pub struct FixedSizeQueue {
    queue: VecDeque<f32>,
    limit: usize,
}

impl FixedSizeQueue {
    pub fn new(limit: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            limit,
        }
    }
    pub fn push(&mut self, value: f32) {
        if self.queue.len() >= self.limit {
            self.queue.pop_front();
        }
        self.queue.push_back(value);
    }
    pub fn avg(&self) -> Option<f32> {
        if self.queue.is_empty() {
            return None;
        }
        let sum: f32 = self.queue.iter().sum();
        Some(sum / self.queue.len() as f32)
    }
}
