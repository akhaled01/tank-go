use crate::net::NetworkClient;
use bevy::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

// Global flag to track if shutdown signal was received
static SHUTDOWN_REQUESTED: AtomicBool = AtomicBool::new(false);

// System to handle graceful shutdown from signals
pub fn handle_shutdown_signal(network: Res<NetworkClient>, mut exit: EventWriter<AppExit>) {
    if SHUTDOWN_REQUESTED.load(Ordering::Relaxed) {
        println!("Shutdown signal received, sending LeaveGame message...");

        // Send LeaveGame message to server
        network.send_leave_game();

        // Give a brief moment for the message to be sent
        std::thread::sleep(std::time::Duration::from_millis(100));

        println!("Gracefully disconnecting from server...");
        exit.write(AppExit::Success);
    }
}

// System to handle app exit events (window close, Cmd+Q, etc.)
pub fn handle_app_exit(mut exit_events: EventReader<AppExit>, network: Res<NetworkClient>) {
    for _exit_event in exit_events.read() {
        println!("App is exiting, sending LeaveGame message...");

        // Send LeaveGame message to server
        network.send_leave_game();

        // Give a brief moment for the message to be sent
        std::thread::sleep(std::time::Duration::from_millis(100));

        println!("Gracefully disconnected from server");
        break; // Only handle the first exit event
    }
}

// Function to set up signal handlers
pub fn setup_signal_handlers() {
    // Set up Ctrl+C handler
    ctrlc::set_handler(move || {
        println!("Received kill signal");
        SHUTDOWN_REQUESTED.store(true, Ordering::Relaxed);
    })
    .expect("Error setting kill signal handler");
}
