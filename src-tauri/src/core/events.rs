use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum AppEvent {
    AppStarted,

    OverlayShown,
    OverlayHidden,

    RecordingStarted,
    RecordingStopped,

    ListeningStarted,
    ListeningStopped,

    ProcessingStarted,
    ProcessingFinished,

    AnswerReceived(String),

    HotkeyPressed,

    ScreenCaptured,

    SettingsUpdated,

    Error(String),
}

pub type EventSender = broadcast::Sender<AppEvent>;
pub type EventReceiver = broadcast::Receiver<AppEvent>;

pub fn create_event_bus() -> (EventSender, EventReceiver) {
    broadcast::channel(100)
}

pub fn publish(sender: &EventSender, event: AppEvent) -> Result<()> {
    sender.send(event)?;
    Ok(())
}
