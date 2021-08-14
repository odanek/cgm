use crate::Vec3;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quaternion<S> {
    pub s: S,
    pub v: Vec3<S>,
}

impl<S> Quaternion<S> {
    #[inline]
    pub const fn new(w: S, xi: S, yj: S, zk: S) -> Quaternion<S> {
        Quaternion::from_sv(w, Vec3::new(xi, yj, zk))
    }

    #[inline]
    pub const fn from_sv(s: S, v: Vec3<S>) -> Quaternion<S> {
        Quaternion { s, v }
    }
}

// impl<S: Float> Zero for Quaternion<S> {
//     const ZERO: Quaternion<S> = Quaternion::from_sv(S::ZERO, Vec3::ZERO);
// }

// impl<S: Float> One for Quaternion<S> {
//     const ONE: Quaternion<S> = Quaternion::from_sv(S::ONE, Vec3::ZERO);
// }
