#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![deny(
    missing_copy_implementations,
    single_use_lifetimes,
    variant_size_differences,
    arithmetic_overflow,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_results,
    unused_lifetimes,
    unused_unsafe,
    useless_ptr_null_checks,
    cenum_impl_drop_cast,
    while_true,
    unused_features,
    absolute_paths_not_starting_with_crate,
    unused_allocation,
    unreachable_code,
    unused_comparisons,
    unused_parens,
    asm_sub_register,
    break_with_label_and_loop,
    bindings_with_variant_name,
    anonymous_parameters,
    bad_asm_style,
    dead_code,
    forgetting_copy_types,
    incomplete_features,
    keyword_idents,
    unused_must_use,
    non_ascii_idents,
    pub_use_of_private_extern_crate,
    exported_private_dependencies,
    clippy::unwrap_used,
)]
#![allow(clippy::module_name_repetitions, clippy::multiple_crate_versions)]

// use std::marker::PhantomData;

// use frclib_core::hal;
// use robots::{RobotCore, RobotCoreImpl, UserRobot};

pub mod math;
pub mod robots;
#[macro_use]
pub mod macros;
pub mod prelude;
pub mod telemetry;
#[cfg(feature = "vendor")]
pub mod vendor;
pub mod runtime;
pub mod io;

pub use frclib_core::units;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventTypes {
    Init,
    Periodic,
    Overrun,
    End,
}

#[cfg(not(frc))]
compile_error!("This crate is only meant to be used with cargo-frc");

#[cfg(test)]
mod example {
    #[test]
    const fn main() {
        use crate::robots::UserRobot;
        use frclib_core::time::Duration;

        #[allow(dead_code)]
        struct Robot {
            name: String,
        }
        impl UserRobot for Robot {
            fn construct() -> Self
            where
                Self: Sized,
            {
                Self {
                    name: "Robot".to_string(),
                }
            }

            fn robot_init(&mut self) {}
            fn robot_periodic(&mut self, _: Duration) {}
            fn robot_teleop_periodic(&mut self, _: Duration) {}
        }
    }
}
