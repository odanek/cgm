use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::{
    Angle, Euler, Float, InnerSpace, Mat3, Mat4, MetricSpace, One, Rad, Vec3, VectorSpace, Zero,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Quat<S> {
    pub s: S,
    pub v: Vec3<S>,
}

impl<S> Quat<S> {
    #[inline]
    pub const fn new(w: S, xi: S, yj: S, zk: S) -> Quat<S> {
        Quat::from_sv(w, Vec3::new(xi, yj, zk))
    }

    #[inline]
    pub const fn from_sv(s: S, v: Vec3<S>) -> Quat<S> {
        Quat { s, v }
    }
}

impl<S: Float> Quat<S> {
    #[inline]
    pub fn conjugate(self) -> Quat<S> {
        Quat::from_sv(self.s, -self.v)
    }

    #[inline]
    pub fn invert(&self) -> Quat<S> {
        self.conjugate() / self.magnitude2()
    }

    pub fn nlerp(self, mut other: Quat<S>, amount: S) -> Quat<S> {
        if self.dot(other) < S::ZERO {
            other = -other;
        }

        (self * (S::ONE - amount) + other * amount).normalize()
    }

    pub fn slerp(self, mut other: Quat<S>, amount: S) -> Quat<S> {
        let mut dot = self.dot(other);
        let dot_threshold = S::ONE; // TODO cgmath uses a different constant

        if dot < S::ZERO {
            other = -other;
            dot = -dot;
        }

        if dot > dot_threshold {
            self.nlerp(other, amount)
        } else {
            let robust_dot = dot.min(S::ONE).max(-S::ONE);

            let theta = Rad::acos(robust_dot);

            let scale1 = Rad::sin(theta * (S::ONE - amount));
            let scale2 = Rad::sin(theta * amount);

            (self * scale1 + other * scale2).normalize()
        }
    }
}

impl<S: Float> Zero for Quat<S> {
    const ZERO: Quat<S> = Quat::from_sv(S::ZERO, Vec3::ZERO);
}

impl<S: Float> One for Quat<S> {
    const ONE: Quat<S> = Quat::from_sv(S::ONE, Vec3::ZERO);
}

impl<S: Float> VectorSpace for Quat<S> {
    type Scalar = S;
}

impl<S: Float> MetricSpace for Quat<S> {
    type Metric = S;

    #[inline]
    fn distance2(self, other: Self) -> S {
        (other - self).magnitude2()
    }
}

impl<S: Float> InnerSpace for Quat<S> {
    #[inline]
    fn dot(self, other: Quat<S>) -> S {
        self.s * other.s + self.v.dot(other.v)
    }
}

impl_operator!(<S: Float> Neg for Quat<S> {
    fn neg(quat) -> Quat<S> {
        Quat::from_sv(-quat.s, -quat.v)
    }
});

impl_operator!(<S: Float> Mul<S> for Quat<S> {
    fn mul(lhs, rhs) -> Quat<S> {
        Quat::from_sv(lhs.s * rhs, lhs.v * rhs)
    }
});

impl_assignment_operator!(<S: Float> MulAssign<S> for Quat<S> {
    fn mul_assign(&mut self, scalar) { self.s *= scalar; self.v *= scalar; }
});

impl_operator!(<S: Float> Div<S> for Quat<S> {
    fn div(lhs, rhs) -> Quat<S> {
        Quat::from_sv(lhs.s / rhs, lhs.v / rhs)
    }
});

impl_assignment_operator!(<S: Float> DivAssign<S> for Quat<S> {
    fn div_assign(&mut self, scalar) { self.s /= scalar; self.v /= scalar; }
});

impl_operator!(<S: Float> Rem<S> for Quat<S> {
    fn rem(lhs, rhs) -> Quat<S> {
        Quat::from_sv(lhs.s % rhs, lhs.v % rhs)
    }
});

impl_assignment_operator!(<S: Float> RemAssign<S> for Quat<S> {
    fn rem_assign(&mut self, scalar) { self.s %= scalar; self.v %= scalar; }
});

impl_operator!(<S: Float> Mul<Vec3<S> > for Quat<S> {
    fn mul(lhs, rhs) -> Vec3<S> {{
        #[allow(clippy::clone_on_copy)]
        let rhs = rhs.clone();
        let two = S::ONE + S::ONE;
        let tmp = lhs.v.cross(rhs) + (rhs * lhs.s);
        (lhs.v.cross(tmp) * two) + rhs
    }}
});

impl_operator!(<S: Float> Add<Quat<S> > for Quat<S> {
    fn add(lhs, rhs) -> Quat<S> {
        Quat::from_sv(lhs.s + rhs.s, lhs.v + rhs.v)
    }
});

impl_assignment_operator!(<S: Float> AddAssign<Quat<S> > for Quat<S> {
    fn add_assign(&mut self, other) { self.s += other.s; self.v += other.v; }
});

impl_operator!(<S: Float> Sub<Quat<S> > for Quat<S> {
    fn sub(lhs, rhs) -> Quat<S> {
        Quat::from_sv(lhs.s - rhs.s, lhs.v - rhs.v)
    }
});

impl_assignment_operator!(<S: Float> SubAssign<Quat<S> > for Quat<S> {
    fn sub_assign(&mut self, other) { self.s -= other.s; self.v -= other.v; }
});

impl_operator!(<S: Float> Mul<Quat<S> > for Quat<S> {
    fn mul(lhs, rhs) -> Quat<S> {
        Quat::new(
            lhs.s * rhs.s - lhs.v.x * rhs.v.x - lhs.v.y * rhs.v.y - lhs.v.z * rhs.v.z,
            lhs.s * rhs.v.x + lhs.v.x * rhs.s + lhs.v.y * rhs.v.z - lhs.v.z * rhs.v.y,
            lhs.s * rhs.v.y + lhs.v.y * rhs.s + lhs.v.z * rhs.v.x - lhs.v.x * rhs.v.z,
            lhs.s * rhs.v.z + lhs.v.z * rhs.s + lhs.v.x * rhs.v.y - lhs.v.y * rhs.v.x,
        )
    }
});

macro_rules! impl_scalar_mul {
    ($S:ident) => {
        impl_operator!(Mul<Quat<$S>> for $S {
            fn mul(scalar, quat) -> Quat<$S> {
                Quat::from_sv(scalar * quat.s, scalar * quat.v)
            }
        });
    };
}

macro_rules! impl_scalar_div {
    ($S:ident) => {
        impl_operator!(Div<Quat<$S>> for $S {
            fn div(scalar, quat) -> Quat<$S> {
                Quat::from_sv(scalar / quat.s, scalar / quat.v)
            }
        });
    };
}

impl_scalar_mul!(f32);
impl_scalar_mul!(f64);
impl_scalar_div!(f32);
impl_scalar_div!(f64);

impl<A> From<Euler<A>> for Quat<A::Unitless>
where
    A: Angle + Into<Rad<A::Unitless>>,
{
    fn from(src: Euler<A>) -> Quat<A::Unitless> {
        let half = A::Unitless::HALF;
        let (s_x, c_x) = Rad::sin_cos(src.x.into() * half);
        let (s_y, c_y) = Rad::sin_cos(src.y.into() * half);
        let (s_z, c_z) = Rad::sin_cos(src.z.into() * half);

        Quat::new(
            -s_x * s_y * s_z + c_x * c_y * c_z,
            s_x * c_y * c_z + s_y * s_z * c_x,
            -s_x * s_z * c_y + s_y * c_x * c_z,
            s_x * s_y * c_z + s_z * c_x * c_y,
        )
    }
}

impl<S: Float> From<Quat<S>> for Mat3<S> {
    #[rustfmt::skip]
    fn from(quat: Quat<S>) -> Mat3<S> {
        let x2 = quat.v.x + quat.v.x;
        let y2 = quat.v.y + quat.v.y;
        let z2 = quat.v.z + quat.v.z;

        let xx2 = x2 * quat.v.x;
        let xy2 = x2 * quat.v.y;
        let xz2 = x2 * quat.v.z;

        let yy2 = y2 * quat.v.y;
        let yz2 = y2 * quat.v.z;
        let zz2 = z2 * quat.v.z;

        let sy2 = y2 * quat.s;
        let sz2 = z2 * quat.s;
        let sx2 = x2 * quat.s;

        Mat3::new(
            S::ONE - yy2 - zz2, xy2 + sz2, xz2 - sy2,
            xy2 - sz2, S::ONE - xx2 - zz2, yz2 + sx2,
            xz2 + sy2, yz2 - sx2, S::ONE - xx2 - yy2,
        )
    }
}

impl<S: Float> From<Quat<S>> for Mat4<S> {
    #[rustfmt::skip]
    fn from(quat: Quat<S>) -> Mat4<S> {
        let x2 = quat.v.x + quat.v.x;
        let y2 = quat.v.y + quat.v.y;
        let z2 = quat.v.z + quat.v.z;

        let xx2 = x2 * quat.v.x;
        let xy2 = x2 * quat.v.y;
        let xz2 = x2 * quat.v.z;

        let yy2 = y2 * quat.v.y;
        let yz2 = y2 * quat.v.z;
        let zz2 = z2 * quat.v.z;

        let sy2 = y2 * quat.s;
        let sz2 = z2 * quat.s;
        let sx2 = x2 * quat.s;

        Mat4::new(
            S::ONE - yy2 - zz2, xy2 + sz2, xz2 - sy2, S::ZERO,
            xy2 - sz2, S::ONE - xx2 - zz2, yz2 + sx2, S::ZERO,
            xz2 + sy2, yz2 - sx2, S::ONE - xx2 - yy2, S::ZERO,
            S::ZERO, S::ZERO, S::ZERO, S::ONE,
        )
    }
}

// TODO: TryFrom?
/// Convert rotation matrix to quaternion
impl<S: Float> From<Mat3<S>> for Quat<S> {
    fn from(mat: Mat3<S>) -> Self {
        let (m00, m01, m02) = mat.x.into_tuple();
        let (m10, m11, m12) = mat.y.into_tuple();
        let (m20, m21, m22) = mat.z.into_tuple();
        if m22 <= S::ZERO {
            let dif10 = m11 - m00;
            let omm22 = S::ONE - m22;
            if dif10 <= S::ZERO {
                let four_xsq = omm22 - dif10;
                let inv4x = S::HALF / four_xsq.sqrt();
                Self::new(
                    four_xsq * inv4x,
                    (m01 + m10) * inv4x,
                    (m02 + m20) * inv4x,
                    (m12 - m21) * inv4x,
                )
            } else {
                let four_ysq = omm22 + dif10;
                let inv4y = S::HALF / four_ysq.sqrt();
                Self::new(
                    (m01 + m10) * inv4y,
                    four_ysq * inv4y,
                    (m12 + m21) * inv4y,
                    (m20 - m02) * inv4y,
                )
            }
        } else {
            let sum10 = m11 + m00;
            let opm22 = S::ONE + m22;
            if sum10 <= S::ZERO {
                let four_zsq = opm22 - sum10;
                let inv4z = S::HALF / four_zsq.sqrt();
                Self::new(
                    (m02 + m20) * inv4z,
                    (m12 + m21) * inv4z,
                    four_zsq * inv4z,
                    (m01 - m10) * inv4z,
                )
            } else {
                let four_wsq = opm22 + sum10;
                let inv4w = S::HALF / four_wsq.sqrt();
                Self::new(
                    (m12 - m21) * inv4w,
                    (m20 - m02) * inv4w,
                    (m01 - m10) * inv4w,
                    four_wsq * inv4w,
                )
            }
        }
    }
}

impl<S: Float> From<Quat<S>> for [S; 4] {
    #[inline]
    fn from(v: Quat<S>) -> Self {
        let (xi, yj, zk, w) = v.into();
        [xi, yj, zk, w]
    }
}

impl<S: Float> From<[S; 4]> for Quat<S> {
    #[inline]
    fn from(v: [S; 4]) -> Quat<S> {
        Quat::new(v[3], v[0], v[1], v[2])
    }
}

impl<S: Float> From<Quat<S>> for (S, S, S, S) {
    #[inline]
    fn from(v: Quat<S>) -> Self {
        let Quat {
            s,
            v: Vec3 { x, y, z },
        } = v;
        (x, y, z, s)
    }
}
