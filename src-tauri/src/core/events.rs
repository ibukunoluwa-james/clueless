use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppEvent {
    AppStarted,
    
    OverlayShown,
    OverlayHidden,

    RecordingStarted,
    RecordingStopped,

    HotkeyPressed,

    ScreenCaptured,

    SettiingsUpdated,
}

use tokio::sync::broadcast;

pub type EventSender = broadcast::Sender<AppEvent>;
pub type EventReceiver = broadcast::Receiver<AppEvent>;

pub fn create_event_bus() -> (EventSender, EventReceiver) {
    broadcast::channel(100)
}

pub fn publish(
    sender: &EventSender,
    event: AppEvent,
) {
    if let Err(err) = sender.send(event) {
            tracing::warn!("No event listeners: {}", err);
        }
}

