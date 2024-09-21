use frclib_core::structure::{FrcStructure, FrcStructureBytes};
use nalgebra::{Quaternion, Unit};


/// A in-crate representation of a [`Unit<Quaternion<f64>>`].
/// This is used to implement [`FrcStructure`] for [`Unit<Quaternion<f64>>`].
/// This is needed for serialization and deserialization of Rotation3d.
#[derive(Copy, Clone, Debug, PartialEq, FrcStructure)]
pub struct FrcQuaternion {
    pub w: f64,
    pub i: f64,
    pub j: f64,
    pub k: f64,
}

impl From<Unit<Quaternion<f64>>> for FrcQuaternion {
    fn from(q: Unit<Quaternion<f64>>) -> Self {
        Self {
            w: q.w,
            i: q.i,
            j: q.j,
            k: q.k,
        }
    }
}

impl From<FrcQuaternion> for Unit<Quaternion<f64>> {
    fn from(q: FrcQuaternion) -> Self {
        Self::from_quaternion(Quaternion::new(q.w, q.i, q.j, q.k))
    }
}
