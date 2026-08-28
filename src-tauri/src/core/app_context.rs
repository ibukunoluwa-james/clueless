use crate::{
    core::{
        events::EventSender,
        state::SharedState,
    },
    services::overlay::OverlayService,
};

pub struct AppContext {
    state: SharedState,
    events: EventSender,
    overlay: OverlayService,
}

impl AppContext {
    pub fn new(
        state: SharedState,
        events: EventSender,
    ) -> Self {
        Self {
            state,
            events,
            overlay: OverlayService::new(),
        }
    }

    pub fn state(&self) -> &SharedState {
        &self.state
    }

    pub fn events(&self) -> &EventSender {
        &self.events
    }

    pub fn overlay(&self) -> &OverlayService {
        &self.overlay
    }
}