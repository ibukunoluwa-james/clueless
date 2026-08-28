use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssistantState {
    Idle,
    Listening,
    Recording,
    Processing,
    Error,
}

#[derive(Debug)]
pub struct AppState {
    pub assistant_state: AssistantState,
    pub overlay_visible: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            assistant_state: AssistantState::Idle,
            overlay_visible: false,
        }
    }
}

impl AppState {
    pub fn assistant_state(&self) -> AssistantState {
        self.assistant_state
    }

    pub fn set_assistant_state(&mut self, state: AssistantState) {
        self.assistant_state = state
    }

    pub fn overlay_visible(&self) -> bool {
        self.overlay_visible
    }
    pub fn set_overlay_visible(&mut self, visible: bool) {
        self.overlay_visible = visible;
    }
}

pub type SharedState = Arc<RwLock<AppState>>;
