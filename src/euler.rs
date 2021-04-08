use crate::{Angle, Mat3, Mat4, One, Zero};

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
