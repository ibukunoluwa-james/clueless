use tauri::{AppHandle, State};

use crate::core::app_context::AppContext;

#[tauri::command]
pub fn show_overlay(
    app: AppHandle,
    context: State<'_, AppContext>,
) -> Result<(), String> {
    context.overlay().show(
        &app,
        context.state(),
        context.events(),
    )
}

#[tauri::command]
pub fn hide_overlay(
    app: AppHandle,
    context: State<'_, AppContext>,
) -> Result<(), String> {
    context.overlay().hide(
        &app,
        context.state(),
        context.events(),
    )
}

#[tauri::command]
pub fn toggle_overlay(
    app: AppHandle,
    context: State<'_, AppContext>,
) -> Result<(), String> {
    context.overlay().toggle(
        &app,
        context.state(),
        context.events(),
    )
}