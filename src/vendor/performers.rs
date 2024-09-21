use std::{error::Error, panic::catch_unwind};

use crate::robots::RobotMode;

#[allow(missing_copy_implementations, missing_debug_implementations)]
/// A performer is a callable that is called at a specific stage of the main loop.
pub struct Performer {
    /// The name of the performer, used for logging.
    pub name: &'static str,
    /// If the performer is essential, if it panics, the program will end.
    pub essential: bool,
    /// The function to be run when the performer is called.
    /// - The function takes a boolean, if true the robot is enabled, if false the robot is disabled(performers on init stage always happen when disabled).
    /// - The function has to be thread-safe(or atleast thread-local).
    /// - The function can panic, if essential is true, the panic will end the program, otherwise it will be caught and logged.
    /// - The function has no guranteed calling order relative to other performers on the same stage.
    pub func: fn(bool) -> Result<(), Box<dyn Error>>,
}
impl Performer {
    /// Create a new performer
    pub const fn new(
        name: &'static str,
        essential: bool,
        func: fn(bool) -> Result<(), Box<dyn Error>>,
    ) -> Self {
        Self {
            name,
            essential,
            func,
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn call(&self, enabled: bool) {
        if self.essential {
            if let Err(e) = (self.func)(enabled) {
                tracing::error!("Performer {} emmitted an error: {}", self.name, e);
            }
        } else {
            let catch = catch_unwind(|| (self.func)(enabled));
            match catch {
                Ok(Err(e)) => {
                    tracing::error!("Performer {} emmitted an error: {}", self.name, e);
                }
                Err(err) => {
                    tracing::error!("Performer {} panic was caught: {:?}", self.name, err);
                }
                _ => {}
            }
        }
    }
}

pub mod stages {
    use super::Performer;
    use linkme::distributed_slice;

    /// Called once after the program has warmed up but before any user code,
    /// users should be able to depend on anything that is initialized here at any point in the program.
    #[distributed_slice]
    pub(crate) static INIT: [Performer];

    /// Called every iteration of the main loop before the user code
    #[distributed_slice]
    pub(crate) static PRE_USER: [Performer];
    /// Called every iteration of the main loop before the user code but after [`PRE_USER`]
    #[distributed_slice]
    pub(crate) static PRE_USER_SIM: [Performer];

    /// Called every iteration of the main loop after the user code
    #[distributed_slice]
    pub(crate) static POST_USER: [Performer];
    /// Called every iteration of the main loop after the user code but before [`POST_USER`]
    #[distributed_slice]
    pub(crate) static POST_USER_SIM: [Performer];
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Stage {
    Init,
    PreUser,
    #[cfg(frc_sim)]
    PreUserSim,
    PostUser,
    #[cfg(frc_sim)]
    PostUserSim,
}
pub(crate) fn call_stage(stage: Stage, mode: RobotMode) {
    match stage {
        Stage::Init => {
            for performer in stages::INIT {
                performer.call(false);
            }
        }
        Stage::PreUser => {
            for performer in stages::PRE_USER {
                performer.call(!mode.is_disabled());
            }
        }
        #[cfg(frc_sim)]
        Stage::PreUserSim => {
            for performer in stages::PRE_USER_SIM {
                performer.call(!mode.is_disabled());
            }
        }
        Stage::PostUser => {
            for performer in stages::POST_USER {
                performer.call(!mode.is_disabled());
            }
        }
        #[cfg(frc_sim)]
        Stage::PostUserSim => {
            for performer in stages::POST_USER_SIM {
                performer.call(!mode.is_disabled());
            }
        }
    }
}
