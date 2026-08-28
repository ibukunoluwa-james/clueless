mod commands;
mod core;
mod services;

use std::sync::{Arc, RwLock};

use core::{
    app_context::AppContext,
    events::create_event_bus,
    state::{AppState, SharedState},
};

use tracing_subscriber::FmtSubscriber;

fn init_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to initialize logger");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logging();

    tracing::info!("Starting Clueless...");

    let app_state: SharedState =
        Arc::new(RwLock::new(AppState::default()));

    let (event_sender, _) = create_event_bus();

    let context = AppContext::new(
        app_state,
        event_sender,
    );

    tauri::Builder::default()
        .manage(context)
        .invoke_handler(tauri::generate_handler![
            commands::app::get_state,
            commands::overlay::show_overlay,
            commands::overlay::hide_overlay,
            commands::overlay::toggle_overlay,
        ])
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            tracing::info!("Application initialized.");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}