use std::ops::{Add, Neg, Sub, Mul, Div};

use frclib_core::{units::{angle::Radian, length::Meter}, structure::{FrcStructure, FrcStructureBytes}};

use num::Float;

#[derive(Copy, Clone, Debug, PartialEq, FrcStructure)]
pub struct Rotation2d {
    pub value: Radian,
    pub(super) sin: f64,
    pub(super) cos: f64,
}
impl Rotation2d {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            value: Radian(0.0),
            sin: 0.0,
            cos: 1.0,
        }
    }
    #[must_use]
    #[inline]
    pub const fn new_angle_const(angle: Radian, sin: f64, cos: f64) -> Self {
        Self {
            value: angle,
            sin,
            cos,
        }
    }
    #[must_use]
    pub fn new_angle(angle: impl Into<Radian>) -> Self {
        let value: Radian = angle.into();
        Self {
            value,
            sin: value.sin().value(),
            cos: value.cos().value(),
        }
    }
    pub fn new_xy(x: impl Into<Meter>, y: impl Into<Meter>) -> Self {
        let x: f64 = x.into().value();
        let y: f64 = y.into().value();
        let magnitude = x.hypot(y);

        let sin = y / magnitude;
        let cos = x / magnitude;

        let value = sin.atan2(cos);
        Self {
            value: value.into(),
            sin,
            cos,
        }
    }

    #[must_use]
    pub fn get_tan(self) -> f64 {
        self.sin / self.cos
    }

    #[must_use]
    pub fn interpolate(self, end_value: Self, t: f64) -> Self {
        self + ((end_value - self) * t.clamp(0.0, 1.0))
    }
}

impl Default for Rotation2d {
    fn default() -> Self {
        Self::new()
    }
}

impl Add for Rotation2d {
    type Output = Self;
    #[allow(clippy::suboptimal_flops)]
    fn add(self, other: Self) -> Self {
        Self::new_xy(
            (self.cos * other.cos) - (self.sin * other.sin),
            (self.cos * other.sin) + (self.sin * other.cos),
        )
    }
}

impl Sub for Rotation2d {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        self + -other
    }
}

impl Neg for Rotation2d {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new_angle(-self.value)
    }
}

impl Mul<f64> for Rotation2d {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::new_angle(self.value.value() * rhs)
    }
}

impl Div<f64> for Rotation2d {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
