use std::ops::{Add, Div, Mul, Sub};

use super::{Rotation2d, Transform2d, Translation2d, Twist2d};

use frclib_core::{units::length::Meter, structure::{FrcStructure, FrcStructureBytes}};

use nalgebra::ComplexField;

/// A structure representing a 2D pose containing translational and rotational elements.
#[derive(Copy, Clone, Debug, PartialEq, FrcStructure)]
pub struct Pose2d {
    pub translation: Translation2d,
    pub rotation: Rotation2d,
}

impl Pose2d {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            translation: Translation2d::new(),
            rotation: Rotation2d::new(),
        }
    }

    #[must_use]
    pub const fn new_trans_rot(translation: Translation2d, rotation: Rotation2d) -> Self {
        Self {
            translation,
            rotation,
        }
    }

    pub fn new_xy_rot(x: impl Into<Meter>, y: impl Into<Meter>, rotation: Rotation2d) -> Self {
        Self::new_trans_rot(Translation2d::new_xy(x, y), rotation)
    }

    #[must_use]
    pub fn transform_by(self, other: Transform2d) -> Self {
        Self::new_trans_rot(
            self.translation + other.translation.rotate_by(self.rotation),
            other.rotation + self.rotation,
        )
    }

    #[must_use]
    pub fn relative_to(self, other: Self) -> Self {
        let transform = Transform2d::new_pose_pose(other, self);
        Self::new_trans_rot(transform.translation, transform.rotation)
    }

    #[must_use]
    pub fn exp(self, twist: Twist2d) -> Self {
        let dx: f64 = twist.dx.into();
        let dy: f64 = twist.dy.into();
        let dtheta: f64 = twist.dtheta.into();

        let sin_theta = dtheta.sin();
        let cos_theta = dtheta.cos();

        let s = if dtheta.abs() < 1e-9 {
            (1.0 / 6.0 * dtheta).mul_add(-dtheta, 1.0)
        } else {
            sin_theta / dtheta
        };

        let c = if dtheta.abs() < 1e-9 {
            0.5 * dtheta
        } else {
            (1.0 - cos_theta) / dtheta
        };

        let transform = Transform2d::new_trans_rot(
            Translation2d::new_xy(Meter(dx.mul_add(s, -dy * c)), Meter(dx.mul_add(c, dy * s))),
            Rotation2d::new_xy(cos_theta, sin_theta),
        );
        self + transform
    }

    #[must_use]
    pub fn log(self, end: Self) -> Twist2d {
        let transform: Self = end.relative_to(self);
        let dtheta: f64 = transform.rotation.value.into();
        let half_dtheta: f64 = dtheta / 2.0;

        let cos_minus_one = transform.rotation.cos - 1.0;

        let halftheta_by_tan_of_halfdtheta = if cos_minus_one.abs() < 1e-9 {
            (1.0 / 12.0 * dtheta).mul_add(-dtheta, 1.0)
        } else {
            -(half_dtheta * transform.rotation.sin) / (cos_minus_one)
        };
        let translation_part =
            transform.translation.rotate_by(Rotation2d::new_xy(
                halftheta_by_tan_of_halfdtheta,
                -half_dtheta,
            )) * ComplexField::hypot(halftheta_by_tan_of_halfdtheta, half_dtheta);

        let dx = translation_part.x;
        let dy = translation_part.y;
        Twist2d::from_parts(dx, dy, dtheta)
    }

    #[must_use]
    pub fn nearest(self, poses: &[Self]) -> Self {
        let mut nearest = poses[0];
        let mut nearest_distance = self.translation.get_distance(nearest.translation);
        for pose in poses {
            let distance = self.translation.get_distance(pose.translation);
            if distance < nearest_distance {
                nearest = *pose;
                nearest_distance = distance;
            }
        }
        nearest
    }

    #[must_use]
    pub fn interpolate(self, end_value: Self, t: f64) -> Self {
        if t < 0.0 {
            self
        } else if t >= 1.0 {
            end_value
        } else {
            let twist = self.log(end_value);
            let scaled_twist = Twist2d::from_parts(
                f64::from(twist.dx) * t,
                f64::from(twist.dy) * t,
                f64::from(twist.dtheta) * t,
            );
            self.exp(scaled_twist)
        }
    }
}

impl Default for Pose2d {
    fn default() -> Self {
        Self::new()
    }
}

impl Add<Transform2d> for Pose2d {
    type Output = Self;

    fn add(self, other: Transform2d) -> Self::Output {
        self.transform_by(other)
    }
}

impl Add for Pose2d {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.transform_by(Transform2d::new_trans_rot(
            other.translation,
            other.rotation,
        ))
    }
}

impl Sub<Transform2d> for Pose2d {
    type Output = Self;

    fn sub(self, rhs: Transform2d) -> Self::Output {
        self.transform_by(rhs.inverse())
    }
}

impl Sub for Pose2d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.relative_to(rhs)
    }
}

impl Mul<f64> for Pose2d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new_trans_rot(self.translation * rhs, self.rotation * rhs)
    }
}

impl Div<f64> for Pose2d {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
