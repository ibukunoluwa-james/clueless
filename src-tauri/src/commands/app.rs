use tauri::State;

use crate::core::{
    app_context::AppContext,
    events::{AppEvent, publish},
    state::SharedState,
};

#[tauri::command]
pub fn test_listening(
    context: State<'_, AppContext>,
) -> Result<(), String> {
    publish(
        context.events(),
        AppEvent::ListeningStarted,
    )
    .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn get_app_state() {}

#[tauri::command]
pub fn health_check() -> &'static str {
    "OK"
}

#[tauri::command]
pub fn get_state(
    state: State<SharedState>,
) -> String {
    let app = state.read().unwrap();

    
    format!("{:#?}", &*app)
}
