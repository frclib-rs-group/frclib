use std::ops::{Add, Div, Mul, Neg, Sub};

use nalgebra::Translation2;
use num::Float;

use crate::math::util::interpolate;

use super::Rotation2d;
use frclib_core::{units::length::{Meter, Distance}, structure::{FrcStructure, FrcStructureBytes}};

#[derive(Copy, Clone, Debug, PartialEq, FrcStructure)]
pub struct Translation2d {
    pub x: Meter,
    pub y: Meter,
}

impl Translation2d {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            x: Meter(0.0),
            y: Meter(0.0)
        }
    }
    #[must_use]
    #[inline]
    pub const fn new_xy_const(x: Meter, y: Meter) -> Self {
        Self { x, y}
    }
    #[must_use]
    pub fn new_xy(x: impl Into<Meter>, y: impl Into<Meter>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
    #[must_use]
    pub fn new_dist_angle(distance: impl Distance, angle: Rotation2d) -> Self {
        let distance: Meter = distance.into();
        Self {
            x: distance * angle.cos,
            y: distance * angle.sin,
        }
    }

    #[must_use]
    pub fn get_distance(self, other: Self) -> Meter {
        let delta_x = other.x - self.x;
        let delta_y = other.y - self.y;
        delta_x.hypot(delta_y)
    }

    #[must_use]
    pub fn get_norm(self) -> Meter {
        self.x.hypot(self.y)
    }

    #[must_use]
    pub fn get_angle(self) -> Rotation2d {
        Rotation2d::new_xy(self.x, self.y)
    }

    #[must_use]
    pub fn rotate_by(self, other: Rotation2d) -> Self {
        Self::new_xy(
            self.x.mul_add(Meter(other.cos), -self.y * other.sin),
            self.x.mul_add(Meter(other.sin), self.y * other.cos),
        )
    }

    #[must_use]
    pub fn nearest(self, translations: &[Self]) -> Self {
        let mut nearest = translations[0];
        let mut nearest_distance = self.get_distance(nearest);
        for translation in translations {
            let distance = self.get_distance(*translation);
            if distance < nearest_distance {
                nearest = *translation;
                nearest_distance = distance;
            }
        }
        nearest
    }

    #[must_use]
    pub const fn get_vector(&self) -> Translation2<Meter> {
        Translation2::new(self.x, self.y)
    }

    #[must_use]
    pub fn interpolate(&self, other: &Self, t: f64) -> Self {
        Self::new_xy(
            interpolate(self.x, other.x, t),
            interpolate(self.y, other.y, t),
        )
    }
}

impl Default for Translation2d {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for Translation2d {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new_xy(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Translation2d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new_xy(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Neg for Translation2d {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new_xy(-self.x, -self.y)
    }
}

impl Mul<f64> for Translation2d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let scalar = Meter(rhs);
        Self::new_xy(self.x * scalar, self.y * scalar)
    }
}

impl Div<f64> for Translation2d {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let scalar = Meter(rhs);
        Self::new_xy(self.x / scalar, self.y / scalar)
    }
}