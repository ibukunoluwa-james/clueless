use tauri::{AppHandle, Manager, WebviewWindow};

use crate::core::{
    events::{publish, AppEvent, EventSender},
    state::SharedState,
};

pub struct OverlayService {}

impl OverlayService {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn show(
        &self,
        app: &AppHandle,
        state: &SharedState,
        events: &EventSender,
    ) -> Result<(), String>{
        let window = self.get_window(app)?;

        window
            .show()
            .map_err(|_| "Failed to show overlay".to_string())?;
        {
            let mut app_state = state
                .write()
                .map_err(|_| "Failed to acquire application state")?;

            app_state.set_overlay_visible(true);

        }
        publish(events, AppEvent::OverlayShown);

        tracing::info!("Overlay Shown");
        Ok(())
    }

    pub fn hide(
        &self, 
        app: &AppHandle, 
        state: &SharedState, 
        events: &EventSender
    ) -> Result<(), String> {
        let window = self.get_window(app)?;

        window
            .hide()
            .map_err(|_| "Failed to hide overlay".to_string())?;
        {
            let mut app_state = state
                .write()
                .map_err(|_| "Failed to acquire application state".to_string())?;
            
            app_state.set_overlay_visible(false);
        }
        publish(events, AppEvent::OverlayHidden);

        tracing::info!("Overlay Hidden");
        Ok(())
    }

    pub fn toggle(
        &self, 
        app: &AppHandle,
        state: &SharedState,
        events: &EventSender,
    ) -> Result<(), String>{
        let visible = {
            let app_state = state
                .read()
                .map_err(|_| "Failed to acquire application state")?;
            app_state.overlay_visible()
        };
        if visible {
            self.hide(app, state, events)
        } else {
            self.show(app, state, events)
        }
    }
    fn get_window(
        &self,
        app: &AppHandle,
    ) -> Result<WebviewWindow, String> {
        app.get_webview_window("overlay")
            .ok_or_else(|| "Overlay window not found".to_string())
    }
}
