use crate::{Angle, Float, Mat3, Mat4, One, Quat, Rad, Zero};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Euler<A> {
    pub x: A, // Pitch
    pub y: A, // Yaw
    pub z: A, // Roll
}

impl<A> Euler<A> {
    pub const fn new(x: A, y: A, z: A) -> Euler<A> {
        Euler { x, y, z }
    }
}

impl<A> From<Euler<A>> for Mat3<A::Unitless>
where
    A: Angle,
{
    #[rustfmt::skip]
    fn from(src: Euler<A>) -> Mat3<A::Unitless> {
        let sx = src.x.sin();
        let cx = src.x.cos();
        let sy = src.y.sin();
        let cy = src.y.cos();
        let sz = src.z.sin();
        let cz = src.z.cos();

        Mat3::new(
            cy * cz, cx * sz + sx * sy * cz, sx * sz - cx * sy * cz,
            -cy * sz, cx * cz - sx * sy * sz, sx * cz + cx * sy * sz,
            sy, -sx * cy, cx * cy,
        )
    }
}

impl<A> From<Euler<A>> for Mat4<A::Unitless>
where
    A: Angle,
{
    #[rustfmt::skip]
    fn from(src: Euler<A>) -> Mat4<A::Unitless> {
        let sx = src.x.sin();
        let cx = src.x.cos();
        let sy = src.y.sin();
        let cy = src.y.cos();
        let sz = src.z.sin();
        let cz = src.z.cos();

        Mat4::new(
            cy * cz, cx * sz + sx * sy * cz, sx * sz - cx * sy * cz, A::Unitless::ZERO,
            -cy * sz, cx * cz - sx * sy * sz, sx * cz + cx * sy * sz, A::Unitless::ZERO,
            sy, -sx * cy, cx * cy, A::Unitless::ZERO,
            A::Unitless::ZERO, A::Unitless::ZERO, A::Unitless::ZERO, A::Unitless::ONE,
        )
    }
}

impl<S: Float> From<Quat<S>> for Euler<Rad<S>> {
    fn from(src: Quat<S>) -> Euler<Rad<S>> {
        let two: S = S::ONE + S::ONE;
        let sig: S = S::HALF;

        let (qw, qx, qy, qz) = (src.s, src.v.x, src.v.y, src.v.z);
        let (sqw, sqx, sqy, sqz) = (qw * qw, qx * qx, qy * qy, qz * qz);

        let unit = sqx + sqz + sqy + sqw;
        let test = qx * qz + qy * qw;

        if test >= sig * unit {
            Euler {
                x: Rad::ZERO,
                y: Rad::HALF_TURN / two,
                z: Rad::atan2(qx, qw) * two,
            }
        } else if test <= -sig * unit {
            Euler {
                x: Rad::ZERO,
                y: -Rad::HALF_TURN / two,
                z: -Rad::atan2(qx, qw) * two,
            }
        } else {
            Euler {
                x: Rad::atan2(two * (-qy * qz + qx * qw), S::ONE - two * (sqx + sqy)),
                y: Rad::asin(two * (qx * qz + qy * qw)),
                z: Rad::atan2(two * (-qx * qy + qz * qw), S::ONE - two * (sqy + sqz)),
            }
        }
    }
}
