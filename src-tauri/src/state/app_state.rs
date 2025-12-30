use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering}
};

#[derive(Clone)]
pub struct AppState {
    running: Arc<AtomicBool>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Relaxed)
    }
}
