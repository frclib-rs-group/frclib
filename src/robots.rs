use std::sync::atomic::AtomicU64;
use std::{fmt::Debug, time::Duration};

use frclib_core::hal::get_hal;
use frclib_core::hal::rt::notifier::NotifierUpdateType;
use frclib_core::time::Instant;
use frclib_core::units::time::{Microsecond, Millisecond};

static PERIODIC_TIME: AtomicU64 = AtomicU64::new(20_000);

/// If time is set to 0, then the periodic loop will run as fast as possible.
pub fn set_periodic_time(time: impl Into<Millisecond>) {
    PERIODIC_TIME.store(
        Microsecond::from(time.into()).value(),
        std::sync::atomic::Ordering::Relaxed,
    );
}

use crate::if_sim;
use crate::vendor::performers::{call_stage, Stage};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RobotMode {
    Disabled = 0,
    Teleop = 1,
    Autonomous = 2,
    Test = 3,
}
impl RobotMode {
    #[must_use]
    pub const fn is_disabled(&self) -> bool {
        matches!(self, Self::Disabled)
    }
    #[must_use]
    pub const fn is_autonomous(&self) -> bool {
        matches!(self, Self::Autonomous)
    }
    #[must_use]
    pub const fn is_teleop(&self) -> bool {
        matches!(self, Self::Teleop)
    }
    #[must_use]
    pub const fn is_test(&self) -> bool {
        matches!(self, Self::Test)
    }
}

/// The core robot trait that is directly used by the frclib runtime.
/// 
/// Most users will not need to implement this trait directly, but instead
/// users will implement the [`UserRobot`] trait which is used by the [`RobotCoreImpl`] struct.
pub trait RobotCore<Robo: UserRobot> {
    fn construct() -> Self
    where
        Self: Sized;

    /// Will be called once at the start of the program after the HAL is initialized.
    fn start(&mut self);

    /// Will be called once at the end of the program before the HAL is deinitialized.
    /// This will even be called if the program panics using [`catch_unwind`](`std::panic::catch_unwind`).
    fn end(&mut self);

    fn get_mode(&self) -> RobotMode;
}

pub trait UserRobot {
    fn construct() -> Self
    where
        Self: Sized;

    /// Only ran once at the start of the program after the HAL is initialized.
    fn robot_init(&mut self);
    /// Ran every cycle.
    fn robot_periodic(&mut self, time_delta: Duration);

    /// Ran upon the robot entering the disabled, no limit to run count.
    fn robot_disabled_init(&mut self) {}
    /// Ran every cycle while the robot is disabled.
    fn robot_disabled_periodic(&mut self, _time_delta: Duration) {}
    /// Ran upon the robot leaving the disabled state, no limit to run count.
    fn robot_disabled_end(&mut self) {}

    /// Ran upon the robot entering the autonomous state, no limit to run count.
    fn robot_autonomous_init(&mut self) {}
    /// Ran every cycle while the robot is in the autonomous state.
    fn robot_autonomous_periodic(&mut self, _time_delta: Duration) {}
    /// Ran upon the robot leaving the autonomous state, no limit to run count.
    fn robot_autonomous_end(&mut self) {}

    /// Ran upon the robot entering the teleop state, no limit to run count.
    fn robot_teleop_init(&mut self) {}
    /// Ran every cycle while the robot is in the teleop state.
    fn robot_teleop_periodic(&mut self, time_delta: Duration);
    /// Ran upon the robot leaving the teleop state, no limit to run count.
    fn robot_teleop_end(&mut self) {}

    /// Ran upon the robot entering the test state, no limit to run count.
    fn robot_test_init(&mut self) {}
    /// Ran every cycle while the robot is in the test state.
    fn robot_test_periodic(&mut self, _time_delta: Duration) {}
    /// Ran upon the robot leaving the test state, no limit to run count.
    fn robot_test_end(&mut self) {}

    /// Ran just after the robot is initialized in simulation.
    fn sim_init(&mut self) {}
    /// Ran every cycle while the robot is in simulation.
    fn sim_periodic(&mut self, _time_delta: Duration) {}
}

pub struct RobotCoreImpl<Robo: UserRobot> {
    user_robot: Robo,
}
impl<T: UserRobot> RobotCore<T> for RobotCoreImpl<T> {
    fn construct() -> Self
    where
        Self: Sized,
    {
        Self {
            user_robot: T::construct(),
        }
    }

    fn start(&mut self) {
        let mut notifier = get_hal()
            .expect("HAL not initialized")
            .notifier_api()
            .new_notifier();
        notifier.update_alarm(NotifierUpdateType::Periodic{
            period: Microsecond(PERIODIC_TIME.load(std::sync::atomic::Ordering::Relaxed)),
            skip_missed: false
        });

        call_stage(Stage::Init, self.get_mode());

        self.user_robot.robot_init();

        #[cfg(frc_sim)]
        self.user_robot.sim_init();

        let mut last_mode = self.get_mode();

        let mut last_mode_periodic_instant = Instant::now();
        let mut last_robot_periodic_instant = Instant::now();
        #[cfg(frc_sim)]
        let mut last_sim_periodic_instant = Instant::now();

        loop {
            let mode = self.get_mode();

            if mode != last_mode {
                match last_mode {
                    RobotMode::Disabled => {
                        self.user_robot.robot_disabled_end();
                    }
                    RobotMode::Autonomous => {
                        self.user_robot.robot_autonomous_end();
                    }
                    RobotMode::Teleop => {
                        self.user_robot.robot_teleop_end();
                    }
                    RobotMode::Test => {
                        self.user_robot.robot_test_end();
                    }
                }
                match mode {
                    RobotMode::Disabled => {
                        self.user_robot.robot_disabled_init();
                    }
                    RobotMode::Autonomous => {
                        self.user_robot.robot_autonomous_init();
                    }
                    RobotMode::Teleop => {
                        self.user_robot.robot_teleop_init();
                    }
                    RobotMode::Test => {
                        self.user_robot.robot_test_init();
                    }
                }
            }

            call_stage(Stage::PreUser, self.get_mode());
            if_sim!(call_stage(Stage::PreUserSim, self.get_mode()););

            {
                let elapsed = last_robot_periodic_instant.elapsed();
                last_robot_periodic_instant = Instant::now();
                self.user_robot.robot_periodic(elapsed);
            }

            {
                let elapsed = last_mode_periodic_instant.elapsed();
                last_mode_periodic_instant = Instant::now();
                match mode {
                    RobotMode::Disabled => {
                        self.user_robot.robot_disabled_periodic(elapsed);
                    }
                    RobotMode::Autonomous => {
                        self.user_robot.robot_autonomous_periodic(elapsed);
                    }
                    RobotMode::Teleop => {
                        self.user_robot.robot_teleop_periodic(elapsed);
                    }
                    RobotMode::Test => {
                        self.user_robot.robot_test_periodic(elapsed);
                    }
                }
            }

            last_mode = mode;

            #[cfg(frc_sim)]
            {
                let elapsed = last_sim_periodic_instant.elapsed();
                last_sim_periodic_instant = Instant::now();
                self.user_robot.sim_periodic(elapsed);
            }

            call_stage(Stage::PostUser, self.get_mode());
            if_sim!{
                call_stage(Stage::PostUserSim, self.get_mode());
            };

            let _ = notifier.wait_for_alarm();
        }
    }

    fn end(&mut self) {}

    fn get_mode(&self) -> RobotMode {
        RobotMode::Disabled
    }
}

impl<T: UserRobot> Debug for RobotCoreImpl<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RobotCoreImpl").finish()
    }
}
impl<T: UserRobot> Default for RobotCoreImpl<T> {
    fn default() -> Self {
        Self {
            user_robot: T::construct(),
        }
    }
}
