use std::panic::{catch_unwind, AssertUnwindSafe};

use frclib_core::hal::{self, HAL, get_hal};

use crate::{robots::{RobotCore, UserRobot}, telemetry::console::setup_tracing_subscriber};

/// Is the entry point for the robot program.
/// 
/// # Panics
/// Panics if the HAL fails to initialize.
#[cfg(frc_real)]
pub fn start<HALDriver: hal::HALDriver, Core: RobotCore<Robo>, Robo: UserRobot>() {
    if let Err(e) = setup_tracing_subscriber() {
        println!("Failed to set up tracing subscriber {e:?}");
        return;
    }

    tracing::info!("Running in real mode");

    tracing::info!("Initializing HAL");

    HAL::init::<HALDriver>();
    let mut core = Core::construct();
    if let Err(e) = catch_unwind(AssertUnwindSafe(|| core.start())) {
        tracing::error!("User code panicked: {:?}", e);
    } else {
        tracing::info!("User code exited normally");
    }
    core.end();
    get_hal().expect("HAL not initialized").cleanup();
}

/// Is the entry point for the robot program.
/// 
/// # Panics
/// Panics if the HAL fails to initialize.
#[cfg(frc_sim)]
pub fn start<HALDriver: hal::SimHALDriver, Core: RobotCore<Robo>, Robo: UserRobot>() {
    if let Err(e) = setup_tracing_subscriber() {
        println!("Failed to set up tracing subscriber {e:?}");
        return;
    }

    tracing::info!("Running in sim mode");

    tracing::info!("Initializing HAL");

    HAL::init_sim::<HALDriver>();
    let mut core = Core::construct();
    if let Err(e) = catch_unwind(AssertUnwindSafe(|| core.start())) {
        tracing::error!("User code panicked: {:?}", e);
    } else {
        tracing::info!("User code exited normally");
    }
    core.end();
    get_hal().expect("HAL not initialized").cleanup();
}