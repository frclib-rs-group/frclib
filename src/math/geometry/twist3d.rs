use frclib_core::{units::{angle::Radian, length::Meter}, structure::{FrcStructure, FrcStructureBytes}};

use super::{Translation3d, Twist2d};

#[derive(Copy, Clone, Debug, PartialEq, FrcStructure)]
pub struct Twist3d {
    pub dx: Meter,
    pub dy: Meter,
    pub dz: Meter,
    pub rx: Radian,
    pub ry: Radian,
    pub rz: Radian,
}

impl Twist3d {
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            dx: Meter(0.0),
            dy: Meter(0.0),
            dz: Meter(0.0),
            rx: Radian(0.0),
            ry: Radian(0.0),
            rz: Radian(0.0),
        }
    }

    #[must_use]
    pub const fn from_parts_const(
        dx: Meter, dy: Meter,
        dz: Meter, rx: Radian,
        ry: Radian, rz: Radian,
    ) -> Self {
        Self {dx,dy,dz,rx,ry,rz,}
    }

    pub fn from_parts(
        dx: impl Into<Meter>,
        dy: impl Into<Meter>,
        dz: impl Into<Meter>,
        rx: impl Into<Radian>,
        ry: impl Into<Radian>,
        rz: impl Into<Radian>,
    ) -> Self {
        Self {
            dx: dx.into(),
            dy: dy.into(),
            dz: dz.into(),
            rx: rx.into(),
            ry: ry.into(),
            rz: rz.into(),
        }
    }

    #[must_use]
    pub const fn translation(self) -> Translation3d {
        Translation3d::from_xyz_const(self.dx, self.dy, self.dz)
    }
}

impl Default for Twist3d {
    fn default() -> Self {
        Self::identity()
    }
}

impl From<Twist2d> for Twist3d {
    fn from(twist: Twist2d) -> Self {
        Self::from_parts(twist.dx, twist.dy, 0.0, 0.0, 0.0, twist.dtheta)
    }
}

impl From<Twist3d> for Twist2d {
    fn from(twist: Twist3d) -> Self {
        Self::from_parts(twist.dx, twist.dy, twist.rz)
    }
}
