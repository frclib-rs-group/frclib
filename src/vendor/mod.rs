use std::path::PathBuf;

pub mod performers;

#[must_use]
pub fn third_party_lib_path() -> PathBuf {
    PathBuf::from(env!("FRC_THIRD_PARTY_LIBS")).join(env!("CARGO_CRATE_NAME"))
}