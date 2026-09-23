use tauri::{ AppHandle, Emitter};

use super::events::{AppEvent, EventReceiver};

pub const APP_EVENT: &str = "app-event";

pub fn start_event_bridge(
    app: AppHandle,
    mut receiver: EventReceiver,
) {
    tauri::async_runtime::spawn(async move {
        while let Ok(event) = receiver.recv().await {
            tracing::debug!("Publishing app event: {:?}", event);

            if let Err(error) = app.emit(APP_EVENT, event) {
                tracing::error!("Failed to emit app event: {error}");
            }
        }
    });
}