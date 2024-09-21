use frclib_core::structure::{FrcStructure, FrcStructureBytes};

use super::{Pose2d, Rotation2d, Translation2d};

#[derive(Copy, Clone, Debug, PartialEq, FrcStructure)]
pub struct Transform2d {
    pub translation: Translation2d,
    pub rotation: Rotation2d,
}

impl Transform2d {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            translation: Translation2d::new(),
            rotation: Rotation2d::new(),
        }
    }

    #[must_use]
    pub fn new_pose_pose(initial: Pose2d, last: Pose2d) -> Self {
        let translation = (last.translation - initial.translation)
            .rotate_by(-initial.rotation);
        let rotation = last.rotation - initial.rotation;
        Self {
            translation,
            rotation,
        }
    }

    #[must_use]
    pub const fn new_trans_rot(translation: Translation2d, rotation: Rotation2d) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    #[must_use]
    pub fn times(&self, scalar: f64) -> Self {
        Self::new_trans_rot(self.translation * scalar, self.rotation * scalar)
    }

    #[must_use]
    pub fn div(&self, scalar: f64) -> Self {
        self.times(1.0 / scalar)
    }

    #[must_use]
    pub fn plus(&self, other: &Self) -> Self {
        Self::new_pose_pose(
            Pose2d::new(),
            Pose2d::new().transform_by(*self).transform_by(*other),
        )
    }

    #[must_use]
    pub fn inverse(&self) -> Self {
        Self::new_trans_rot(
            -self.translation
                .rotate_by(-self.rotation),
            -self.rotation,
        )
    }
}

impl Default for Transform2d {
    fn default() -> Self {
        Self::new()
    }
}
