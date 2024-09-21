use std::ops::{Add, Div, Mul, Neg, Sub};

use nalgebra::{
    ComplexField, Quaternion,
    Rotation3, Unit, UnitQuaternion, Vector3,
};

use frclib_core::{units::angle::{Radian, Angle}, structure::FrcStructure};

use super::{Rotation2d, quaternion::FrcQuaternion};


/// A 3d representation of a rotation. <br>
/// This is a wrapper around [`UnitQuaternion<f64>`](nalgebra::UnitQuaternion) with a wpilib-like API.
/// 
/// # Constructors
/// - `const` [`Rotation3d::identity()`]
/// - `const` [`Rotation3d::from_quaternion_unchecked()`]
/// - [`Rotation3d::from_quaternion()`]
/// - [`Rotation3d::from_angles()`]
/// - [`Rotation3d::from_axis_angles()`]
/// - [`Rotation3d::from_rotation_vector()`]
/// - [`Rotation3d::from_rotation_matrix()`]
/// - `todo` [`Rotation3d::from_first_last()`]
/// 
/// # Operators
/// - `Rotation3d::add() -> Self`
/// - `Rotation3d::sub() -> Self`
/// - `Rotation3d::neg() -> Self`
/// - `Rotation3d::mul::<f64>() -> Self`
/// - `Rotation3d::div::<f64>() -> Self`
/// 
/// # Examples
/// ```
/// use frclib::units::angle::Radian;
/// use frclib::math::geometry::Rotation3d;
/// use frclib::math::nalgebra::Quaternion;
/// 
/// /// A const Rotation3d with no rotation.
/// const ROTATION: Rotation3d = Rotation3d::from_quaternion_unchecked(Quaternion::new(1.0, 0.0, 0.0, 0.0));
/// 
/// pub fn main() {
///    let rotation = Rotation3d::new_angles(0.0, Radian::from(0.0), 180);
///    assert_eq!(rotation.x(), Radian::from(0.0));
///    assert_eq!(rotation.y(), Radian::from(0.0));
///    assert_eq!(rotation.z(), Radian::from(180.0));
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotation3d {
    pub q: UnitQuaternion<f64>,
}

// Constructors
impl Rotation3d {
    /// Creates a new Rotation3d with no rotation.
    #[must_use]
    pub const fn identity() -> Self {
        Self {
            q: Unit::new_unchecked(Quaternion::new(1.0, 0.0, 0.0, 0.0)),
        }
    }

    /// Creates a new Rotation3d from a quaternion. This is unsafe because the quaternion is not
    /// guaranteed to be normalized.
    /// 
    /// # Safety
    /// This must be constucted from a normalized quaternion.
    #[must_use]
    pub const fn from_quaternion_unchecked(q: Quaternion<f64>) -> Self {
        Self { q: Unit::new_unchecked(q) }
    }

    /// Creates a new Rotation3d from a quaternion.
    /// The quaternion is normalized before being used.
    #[must_use]
    pub fn from_quaternion(q: Quaternion<f64>) -> Self {
        Self { q: UnitQuaternion::new_normalize(q), }
    }

    /// Creates a new Rotation3d from euler angles.
    #[must_use]
    pub fn from_angles(roll: impl Angle, pitch: impl Angle, yaw: impl Angle) -> Self {
        Self {
            q: UnitQuaternion::from_euler_angles(
                roll.into().value(),
                pitch.into().value(),
                yaw.into().value(),
            ),
        }
    }

    /// Creates a new Rotation3d from a rotation vector.
    #[must_use]
    pub fn from_rotation_vector(rvec: Vector3<f64>) -> Self {
        Self::from_axis_angle(rvec, Radian(rvec.norm()))
    }

    /// Creates a new Rotation3d from a rotation vector and an angle
    #[must_use]
    fn from_axis_angle(axis: Vector3<f64>, angle: impl Angle) -> Self {
        Self {
            q: UnitQuaternion::from_axis_angle(&Unit::new_normalize(axis), angle.into().value()),
        }
    }

    /// Creates a new Rotation3d from a rotation matrix.
    #[must_use]
    pub fn from_rotation_matrix(matrix: Rotation3<f64>) -> Self {
        Self {q: UnitQuaternion::from_rotation_matrix(&matrix), }
    }

    #[must_use]
    /// NOT YET IMPLEMENTED
    /// # Panics
    /// This function panics if used in any way.
    pub fn from_first_last(_initial: Vector3<f64>, _last: Vector3<f64>) -> Self {
        todo!()
    }
}

// Methods
impl Rotation3d {
    #[must_use]
    /// Returns rotation around the x axis. Equivalent to [`Rotation3d::roll()`].
    pub fn x(&self) -> Radian {
        let w = self.q.w;
        let x = self.q.i;
        let y = self.q.j;
        let z = self.q.k;

        f64::atan2(
            2.0 * w.mul_add(x, y * z),
            2.0f64.mul_add((-x).mul_add(x, y * y), 1.0),
        )
        .into()
    }

    #[must_use]
    /// Returns rotation around the y axis. Equivalent to [`Rotation3d::pitch()`].
    pub fn y(&self) -> Radian {
        let w = self.q.w;
        let x = self.q.i;
        let y = self.q.j;
        let z = self.q.k;

        let ratio = 2.0 * w.mul_add(y, -z * x);
        if ratio.abs() >= 1.0 {
            f64::copysign(std::f64::consts::PI / 2.0, ratio).into()
        } else {
            f64::asin(ratio).into()
        }
    }

    #[must_use]
    /// Returns rotation around the z axis. Equivalent to [`Rotation3d::yaw()`].
    pub fn z(self) -> Radian {
        let w = self.q.w;
        let x = self.q.i;
        let y = self.q.j;
        let z = self.q.k;

        f64::atan2(
            2.0 * w.mul_add(z, x * y),
            2.0f64.mul_add((-y).mul_add(y, z * z), 1.0),
        )
        .into()
    }

    #[must_use]
    #[inline]
    /// Returns the yaw of the rotation. Equivalent to [`Rotation3d::z()`].
    pub fn yaw(self) -> Radian {
        self.z()
    }

    #[must_use]
    #[inline]
    /// Returns the pitch of the rotation. Equivalent to [`Rotation3d::y()`].
    pub fn pitch(self) -> Radian {
        self.y()
    }

    #[must_use]
    #[inline]
    /// Returns the roll of the rotation. Equivalent to [`Rotation3d::x()`].
    pub fn roll(self) -> Radian {
        self.x()
    }

    #[must_use]
    pub fn get_axis(self) -> Vector3<Radian> {
        self.q.axis().map_or_else(
            || Vector3::new(0.0.into(), 0.0.into(), 0.0.into()),
            |axis: Unit<Vector3<f64>>| Vector3::new(axis.x.into(), axis.y.into(), axis.z.into()),
        )
    }

    #[must_use]
    pub fn get_angle(self) -> Radian {
        self.q.angle().into()
    }

    #[must_use]
    pub fn interpolate(self, end_value: Self, t: f64) -> Self {
        self + ((end_value - self) * t.clamp(0.0, 1.0))
    }
}

impl Default for Rotation3d {
    fn default() -> Self {
        Self::identity()
    }
}

impl From<Rotation2d> for Rotation3d {
    fn from(r: Rotation2d) -> Self {
        Self::from_angles(Radian(0.0), Radian(0.0), r.value)
    }
}

impl From<Rotation3d> for Rotation2d {
    fn from(r: Rotation3d) -> Self {
        Self::new_angle(r.z())
    }
}

impl Add for Rotation3d {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: Self) -> Self::Output {
        Self { q: self.q * rhs.q }
    }
}

impl Sub for Rotation3d {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, rhs: Self) -> Self::Output {
        Self { q: self.q * rhs.q.inverse() }
    }
}

impl Neg for Rotation3d {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            q: self.q.inverse(),
        }
    }
}

impl Mul<f64> for Rotation3d {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        if self.q.w >= 0.0 {
            Self::from_axis_angle(
                Vector3::new(self.q.i, self.q.j, self.q.k),
                Radian(2.0 * rhs * ComplexField::acos(self.q.w)),
            )
        } else {
            Self::from_axis_angle(
                Vector3::new(-self.q.i, -self.q.j, -self.q.k),
                Radian(2.0 * rhs * ComplexField::acos(-self.q.w)),
            )
        }
    }
}

impl Div<f64> for Rotation3d {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl frclib_core::structure::FrcStructure for Rotation3d {
    const SIZE: usize = FrcQuaternion::SIZE;
    const SCHEMA_SUPPLIER: fn() -> String = || "FrcQuaternion q;".to_owned();
    const TYPE: &'static str = stringify!(Rotation3d);
    fn pack(&self, buffer: &mut Vec<u8>) {
        frclib_core::structure::FrcStructure::pack(&FrcQuaternion::from(self.q), buffer);
    }
    fn unpack(buffer: &mut std::io::Cursor<&[u8]>) -> Self {
        Self {
            q: FrcQuaternion::unpack(buffer).into(),
        }
    }
}
frclib_core::structure::inventory::submit! {
    Rotation3d::DESCRIPTION
}