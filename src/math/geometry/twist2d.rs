use frclib_core::{units::{angle::Radian, length::Meter}, structure::{FrcStructure, FrcStructureBytes}};

#[derive(Copy, Clone, Debug, PartialEq, FrcStructure)]
pub struct Twist2d {
    pub dx: Meter,
    pub dy: Meter,
    pub dtheta: Radian,
}

impl Twist2d {
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            dx: Meter(0.0),
            dy: Meter(0.0),
            dtheta: Radian(0.0),
        }
    }

    #[must_use]
    pub const fn from_parts_const(
        dx: Meter,
        dy: Meter,
        dtheta: Radian,
    ) -> Self {
        Self {dx,dy,dtheta,}
    }

    #[must_use]
    pub fn from_parts(
        dx: impl Into<Meter>,
        dy: impl Into<Meter>,
        dtheta: impl Into<Radian>,
    ) -> Self {
        Self {
            dx: dx.into(),
            dy: dy.into(),
            dtheta: dtheta.into(),
        }
    }
}

impl Default for Twist2d {
    fn default() -> Self {
        Self::identity()
    }
}
