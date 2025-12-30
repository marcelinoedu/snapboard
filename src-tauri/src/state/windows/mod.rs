use std::sync::Mutex;

#[derive(Debug, Clone, Copy)]
pub struct OverlayWindowState {
    pub visible: bool,
    pub had_focus: bool,
}

impl Default for OverlayWindowState {
    fn default() -> Self {
        Self {
            visible: false,
            had_focus: false,
        }
    }
}

pub struct WindowState {
    overlay: Mutex<OverlayWindowState>,
}

impl WindowState {
    pub fn new() -> Self {
        Self {
            overlay: Mutex::new(OverlayWindowState::default()),
        }
    }

    pub fn snapshot(&self) -> OverlayWindowState {
        *self.overlay.lock().unwrap()
    }

    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(&mut OverlayWindowState),
    {
        let mut state = self.overlay.lock().unwrap();
        f(&mut state);
    }
}
