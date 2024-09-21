use std::ops::{Add, Div, Mul, Neg, Sub};

use nalgebra::Quaternion;

use frclib_core::{units::length::Meter, structure::{FrcStructure, FrcStructureBytes}};
use num::Float;

use crate::math::util::interpolate;

use super::{Rotation3d, Translation2d};

#[derive(Copy, Clone, Debug, PartialEq, FrcStructure)]
pub struct Translation3d {
    pub x: Meter,
    pub y: Meter,
    pub z: Meter,
}

impl Translation3d {
    #[must_use]
    pub const fn identity() -> Self {
        Self{
            x: Meter(0.0),
            y: Meter(0.0),
            z: Meter(0.0),
        }
    }

    #[must_use]
    pub const fn from_xyz_const(x: Meter, y: Meter, z: Meter) -> Self {
        Self { x, y, z }
    }

    #[must_use]
    pub fn from_xyz(x: impl Into<Meter>, y: impl Into<Meter>, z: impl Into<Meter>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }

    #[must_use]
    pub fn new_dist_angle(dist: impl Into<Meter>, angle: Rotation3d) -> Self {
        Self::from_xyz(dist, 0.0, 0.0).rotate_by(angle)
    }

    #[must_use]
    #[allow(clippy::suboptimal_flops)]
    pub fn get_distance(&self, other: &Self) -> Meter {
        Meter(Float::sqrt(
            (other.x - self.x).value().powi(2)
            + (other.y - self.y).value().powi(2)
            + (other.z - self.z).value().powi(2)
        ))
    }

    #[must_use]
    #[allow(clippy::suboptimal_flops)]
    pub fn get_norm(&self) -> Meter {
        Meter(Float::sqrt(
            self.x.value().powi(2)
            + self.y.value().powi(2)
            + self.z.value().powi(2)
        ))
    }

    /// Rotates the translation3d by the given rotation and returns the new translation3d.
    ///
    /// # Warning
    /// If the rotation is zero, this function will return a translation3d with the same values as the original.
    #[must_use]
    pub fn rotate_by(&self, other: Rotation3d) -> Self {
        let Some(other_quat) = other.q.try_inverse() else {
            //if other.q is zero do nothing
            return *self;
        };
        let p = Quaternion::new(0.0f64, self.x.value(), self.y.value(), self.z.value());
        let qprime: Quaternion<f64> = other.q.quaternion() * p * other_quat;
        Self::from_xyz(qprime.i, qprime.j, qprime.k)
    }

    /// Interpolates between this translation3d and the end translation3d by the given percent.
    #[must_use]
    pub fn interpolate(&self, end_value: Self, t: f64) -> Self {
        Self::from_xyz(
            interpolate(self.x, end_value.x, t),
            interpolate(self.y, end_value.y, t),
            interpolate(self.z, end_value.z, t),
        )
    }
}

impl Default for Translation3d {
    fn default() -> Self {
        Self::identity()
    }
}

impl From<Translation2d> for Translation3d {
    fn from(translation: Translation2d) -> Self {
        Self::from_xyz(translation.x, translation.y, 0.0)
    }
}

impl From<Translation3d> for Translation2d {
    fn from(translation: Translation3d) -> Self {
        Self::new_xy(translation.x, translation.y)
    }
}

impl Add for Translation3d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_xyz(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Translation3d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_xyz(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Translation3d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::from_xyz(
            self.x.value() * rhs,
            self.y.value() * rhs,
            self.z.value() * rhs,
        )
    }
}

impl Div<f64> for Translation3d {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self.mul(1.0 / rhs)
    }
}

impl Neg for Translation3d {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_xyz(-self.x, -self.y, -self.z)
    }
}
