use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::{
    Angle, Euler, Float, InnerSpace, Mat3, Mat4, MetricSpace, One, Rad, Vec3, VectorSpace, Zero,
};

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

impl<S: Float> Quaternion<S> {
    #[inline]
    pub fn conjugate(self) -> Quaternion<S> {
        Quaternion::from_sv(self.s, -self.v)
    }

    pub fn nlerp(self, mut other: Quaternion<S>, amount: S) -> Quaternion<S> {
        if self.dot(other) < S::ZERO {
            other = -other;
        }

        (self * (S::ONE - amount) + other * amount).normalize()
    }

    pub fn slerp(self, mut other: Quaternion<S>, amount: S) -> Quaternion<S> {
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

impl<S: Float> Zero for Quaternion<S> {
    const ZERO: Quaternion<S> = Quaternion::from_sv(S::ZERO, Vec3::ZERO);
}

impl<S: Float> One for Quaternion<S> {
    const ONE: Quaternion<S> = Quaternion::from_sv(S::ONE, Vec3::ZERO);
}

impl<S: Float> VectorSpace for Quaternion<S> {
    type Scalar = S;
}

impl<S: Float> MetricSpace for Quaternion<S> {
    type Metric = S;

    #[inline]
    fn distance2(self, other: Self) -> S {
        (other - self).magnitude2()
    }
}

impl<S: Float> InnerSpace for Quaternion<S> {
    #[inline]
    fn dot(self, other: Quaternion<S>) -> S {
        self.s * other.s + self.v.dot(other.v)
    }
}

impl<A> From<Euler<A>> for Quaternion<A::Unitless>
where
    A: Angle + Into<Rad<A::Unitless>>,
{
    fn from(src: Euler<A>) -> Quaternion<A::Unitless> {
        let half = A::Unitless::ONE / (A::Unitless::ONE + A::Unitless::ONE); // TODO
        let (s_x, c_x) = Rad::sin_cos(src.x.into() * half);
        let (s_y, c_y) = Rad::sin_cos(src.y.into() * half);
        let (s_z, c_z) = Rad::sin_cos(src.z.into() * half);

        Quaternion::new(
            -s_x * s_y * s_z + c_x * c_y * c_z,
            s_x * c_y * c_z + s_y * s_z * c_x,
            -s_x * s_z * c_y + s_y * c_x * c_z,
            s_x * s_y * c_z + s_z * c_x * c_y,
        )
    }
}

impl_operator!(<S: Float> Neg for Quaternion<S> {
    fn neg(quat) -> Quaternion<S> {
        Quaternion::from_sv(-quat.s, -quat.v)
    }
});

impl_operator!(<S: Float> Mul<S> for Quaternion<S> {
    fn mul(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s * rhs, lhs.v * rhs)
    }
});

impl_assignment_operator!(<S: Float> MulAssign<S> for Quaternion<S> {
    fn mul_assign(&mut self, scalar) { self.s *= scalar; self.v *= scalar; }
});

impl_operator!(<S: Float> Div<S> for Quaternion<S> {
    fn div(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s / rhs, lhs.v / rhs)
    }
});

impl_assignment_operator!(<S: Float> DivAssign<S> for Quaternion<S> {
    fn div_assign(&mut self, scalar) { self.s /= scalar; self.v /= scalar; }
});

impl_operator!(<S: Float> Rem<S> for Quaternion<S> {
    fn rem(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s % rhs, lhs.v % rhs)
    }
});

impl_assignment_operator!(<S: Float> RemAssign<S> for Quaternion<S> {
    fn rem_assign(&mut self, scalar) { self.s %= scalar; self.v %= scalar; }
});

impl_operator!(<S: Float> Mul<Vec3<S> > for Quaternion<S> {
    fn mul(lhs, rhs) -> Vec3<S> {{        
        let rhs = rhs.clone();
        let two = S::ONE + S::ONE;
        let tmp = lhs.v.cross(rhs) + (rhs * lhs.s);
        (lhs.v.cross(tmp) * two) + rhs
    }}
});

impl_operator!(<S: Float> Add<Quaternion<S> > for Quaternion<S> {
    fn add(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s + rhs.s, lhs.v + rhs.v)
    }
});

impl_assignment_operator!(<S: Float> AddAssign<Quaternion<S> > for Quaternion<S> {
    fn add_assign(&mut self, other) { self.s += other.s; self.v += other.v; }
});

impl_operator!(<S: Float> Sub<Quaternion<S> > for Quaternion<S> {
    fn sub(lhs, rhs) -> Quaternion<S> {
        Quaternion::from_sv(lhs.s - rhs.s, lhs.v - rhs.v)
    }
});

impl_assignment_operator!(<S: Float> SubAssign<Quaternion<S> > for Quaternion<S> {
    fn sub_assign(&mut self, other) { self.s -= other.s; self.v -= other.v; }
});

impl_operator!(<S: Float> Mul<Quaternion<S> > for Quaternion<S> {
    fn mul(lhs, rhs) -> Quaternion<S> {
        Quaternion::new(
            lhs.s * rhs.s - lhs.v.x * rhs.v.x - lhs.v.y * rhs.v.y - lhs.v.z * rhs.v.z,
            lhs.s * rhs.v.x + lhs.v.x * rhs.s + lhs.v.y * rhs.v.z - lhs.v.z * rhs.v.y,
            lhs.s * rhs.v.y + lhs.v.y * rhs.s + lhs.v.z * rhs.v.x - lhs.v.x * rhs.v.z,
            lhs.s * rhs.v.z + lhs.v.z * rhs.s + lhs.v.x * rhs.v.y - lhs.v.y * rhs.v.x,
        )
    }
});

macro_rules! impl_scalar_mul {
    ($S:ident) => {
        impl_operator!(Mul<Quaternion<$S>> for $S {
            fn mul(scalar, quat) -> Quaternion<$S> {
                Quaternion::from_sv(scalar * quat.s, scalar * quat.v)
            }
        });
    };
}

macro_rules! impl_scalar_div {
    ($S:ident) => {
        impl_operator!(Div<Quaternion<$S>> for $S {
            fn div(scalar, quat) -> Quaternion<$S> {
                Quaternion::from_sv(scalar / quat.s, scalar / quat.v)
            }
        });
    };
}

impl_scalar_mul!(f32);
impl_scalar_mul!(f64);
impl_scalar_div!(f32);
impl_scalar_div!(f64);

impl<S: Float> From<Quaternion<S>> for Mat3<S> {
    #[rustfmt::skip]
    fn from(quat: Quaternion<S>) -> Mat3<S> {
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

impl<S: Float> From<Quaternion<S>> for Mat4<S> {
    #[rustfmt::skip]
    fn from(quat: Quaternion<S>) -> Mat4<S> {
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

impl<S: Float> From<Quaternion<S>> for [S; 4] {
    #[inline]
    fn from(v: Quaternion<S>) -> Self {
        let (xi, yj, zk, w) = v.into();
        [xi, yj, zk, w]
    }
}

impl<S: Float> From<[S; 4]> for Quaternion<S> {
    #[inline]
    fn from(v: [S; 4]) -> Quaternion<S> {
        Quaternion::new(v[3], v[0], v[1], v[2])
    }
}

impl<S: Float> From<Quaternion<S>> for (S, S, S, S) {
    #[inline]
    fn from(v: Quaternion<S>) -> Self {
        let Quaternion {
            s,
            v: Vec3 { x, y, z },
        } = v;
        (x, y, z, s)
    }
}
