use tauri::State;

use crate::core::{
    events::{AppEvent, EventSender},
    state::SharedState,
};
use crate::core::events::publish;

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
