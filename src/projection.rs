use crate::{Angle, Float, Mat4, Rad};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Perspective<S> {
    pub fovy: Rad<S>,
    pub aspect: S,
    pub near: S,
    pub far: S,
}

impl<S: Float> From<Perspective<S>> for Mat4<S> {
    #[rustfmt::skip]
    fn from(persp: Perspective<S>) -> Mat4<S> {
        let half_fov = persp.fovy * S::HALF;
        let f = half_fov.cos() / half_fov.sin();
        let d = persp.far - persp.near;

        Mat4::new(
            f / persp.aspect, S::ZERO, S::ZERO, S::ZERO,
            S::ZERO, -f, S::ZERO, S::ZERO,
            S::ZERO, S::ZERO, -persp.far / d, -S::ONE,
            S::ZERO, S::ZERO, -(persp.far * persp.near) / d, S::ZERO,
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ortho<S> {
    pub left: S,
    pub right: S,
    pub bottom: S,
    pub top: S,
    pub near: S,
    pub far: S,
}

impl<S: Float> From<Ortho<S>> for Mat4<S> {
    #[rustfmt::skip]
    fn from(ortho: Ortho<S>) -> Mat4<S> {
        let x = ortho.right - ortho.left;
        let y = ortho.top - ortho.bottom;
        let z = ortho.far - ortho.near;
        let two = S::ONE + S::ONE;

        Mat4::new(
            two / x, S::ZERO, S::ZERO, S::ZERO,
            S::ZERO, two / y, S::ZERO, S::ZERO,
            S::ZERO, S::ZERO, -two / z, S::ZERO,
            (ortho.left + ortho.right) / -x, (ortho.bottom + ortho.top) / -y, (ortho.near + ortho.far) / -z, S::ONE,
        )
    }
}
