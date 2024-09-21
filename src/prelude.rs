
//core
pub use frclib_core;
pub use frclib_core::structure::FrcStructure;
pub use frclib_core::time;
pub use frclib_core::value::*;
pub use frclib_core::hal;

pub use crate::math;

pub use num::{Float, Integer, Num, NumCast, One, Signed, ToPrimitive, Zero};

pub use crate::telemetry::{log, log_with_timestamp};

pub use cfg_if::cfg_if;