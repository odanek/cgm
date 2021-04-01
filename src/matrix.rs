use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::{Angle, Float, InnerSpace, Num, One, Rad, Vec2, Vec3, Vec4, Zero};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Mat2<S> {
    pub x: Vec2<S>,
    pub y: Vec2<S>,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Mat3<S> {
    pub x: Vec3<S>,
    pub y: Vec3<S>,
    pub z: Vec3<S>,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Mat4<S> {
    pub x: Vec4<S>,
    pub y: Vec4<S>,
    pub z: Vec4<S>,
    pub w: Vec4<S>,
}

pub trait Matrix {
    type Row;
    type Column;
    type Transpose;

    fn row(&self, r: usize) -> Self::Row;
    fn column(&self, c: usize) -> Self::Column;
    fn transpose(&self) -> Self::Transpose;
}

impl<S: Zero + One> Mat2<S> {
    pub const IDENTITY: Mat2<S> = Mat2::from_cols(Vec2::X, Vec2::Y);
}

impl<S: Zero> Zero for Mat2<S> {
    const ZERO: Mat2<S> = Mat2::from_cols(Vec2::ZERO, Vec2::ZERO);
}

impl<S> Mat2<S> {
    #[inline]
    pub const fn new(c0r0: S, c0r1: S, c1r0: S, c1r1: S) -> Mat2<S> {
        Self::from_cols(Vec2::new(c0r0, c0r1), Vec2::new(c1r0, c1r1))
    }

    #[inline]
    pub const fn from_cols(x: Vec2<S>, y: Vec2<S>) -> Mat2<S> {
        Mat2 { x, y }
    }
}

impl<S: Float> Mat2<S> {
    #[inline]
    pub fn from_translation(t: S) -> Mat2<S> {
        Mat2::new(S::ONE, S::ZERO, t, S::ONE)
    }

    #[inline]
    pub fn from_scale(value: S) -> Mat2<S> {
        Mat2::new(value, S::ZERO, S::ZERO, value)
    }

    #[inline]
    pub fn from_nonuniform_scale(x: S, y: S) -> Mat2<S> {
        Mat2::new(x, S::ZERO, S::ZERO, y)
    }

    #[inline]
    pub fn from_rotation<A: Into<Rad<S>>>(theta: A) -> Mat2<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();
        Mat2::new(c, s, -s, c)
    }
}

impl<S: Num> Matrix for Mat2<S> {
    type Row = Vec2<S>;
    type Column = Vec2<S>;
    type Transpose = Mat2<S>;

    #[inline]
    fn row(&self, r: usize) -> Vec2<S> {
        match r {
            0 => Vec2::new(self.x.x, self.y.x),
            1 => Vec2::new(self.x.y, self.y.y),
            _ => panic!("Invalid row index"),
        }
    }

    #[inline]
    fn column(&self, r: usize) -> Vec2<S> {
        match r {
            0 => self.x,
            1 => self.y,
            _ => panic!("Invalid column index"),
        }
    }

    fn transpose(&self) -> Mat2<S> {
        Mat2::new(self.x.x, self.y.x, self.x.y, self.y.y)
    }
}

impl_operator!(<S: Float> Mul<Mat2<S>> for Mat2<S> {
    fn mul(lhs, rhs) -> Mat2<S> {
        Mat2::new(
            lhs.row(0).dot(rhs.column(0)), lhs.row(1).dot(rhs.column(0)),
            lhs.row(0).dot(rhs.column(1)), lhs.row(1).dot(rhs.column(1)),
        )
    }
});

impl<S: Zero> Zero for Mat3<S> {
    const ZERO: Mat3<S> = Mat3::from_cols(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO);
}

impl<S: Zero + One> Mat3<S> {
    pub const IDENTITY: Mat3<S> = Mat3::from_cols(Vec3::X, Vec3::Y, Vec3::Z);
}

impl<S> Mat3<S> {
    #[inline]
    #[rustfmt::skip]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        c0r0:S, c0r1:S, c0r2:S,
        c1r0:S, c1r1:S, c1r2:S,
        c2r0:S, c2r1:S, c2r2:S,
    ) -> Mat3<S> {
        Mat3::from_cols(
            Vec3::new(c0r0, c0r1, c0r2),
            Vec3::new(c1r0, c1r1, c1r2),
            Vec3::new(c2r0, c2r1, c2r2),
        )
    }

    #[inline]
    pub const fn from_cols(x: Vec3<S>, y: Vec3<S>, z: Vec3<S>) -> Mat3<S> {
        Mat3 { x, y, z }
    }
}

impl<S: Float> Mat3<S> {
    #[inline]
    #[rustfmt::skip]
    pub fn from_translation(v: Vec2<S>) -> Mat3<S> {
        Mat3::new(
            S::ONE, S::ZERO, S::ZERO,
            S::ZERO, S::ONE, S::ZERO,
            v.x, v.y, S::ONE,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_scale(value: S) -> Mat3<S> {
        Mat3::new(
            value, S::ZERO, S::ZERO,
            S::ZERO, value, S::ZERO,
            S::ZERO, S::ZERO, value,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_nonuniform_scale(x: S, y: S, z: S) -> Mat3<S> {
        Mat3::new(
            x, S::ZERO, S::ZERO,
            S::ZERO, y, S::ZERO,
            S::ZERO, S::ZERO, z,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation<A: Into<Rad<S>>>(axis: Vec3<S>, theta: A) -> Mat3<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();
        let mc = S::ONE - c;

        Mat3::new(
            mc * axis.x * axis.x + c,
            mc * axis.x * axis.y + s * axis.z,
            mc * axis.x * axis.z - s * axis.y,

            mc * axis.x * axis.y - s * axis.z,
            mc * axis.y * axis.y + c,
            mc * axis.y * axis.z + s * axis.x,

            mc * axis.x * axis.z + s * axis.y,
            mc * axis.y * axis.z - s * axis.x,
            mc * axis.z * axis.z + c,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation_x<A: Into<Rad<S>>>(theta: A) -> Mat3<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();

        Mat3::new(
            S::ONE, S::ZERO, S::ZERO,
            S::ZERO, c, s,
            S::ZERO, -s, c,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation_y<A: Into<Rad<S>>>(theta: A) -> Mat3<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();

        Mat3::new(
            c, S::ZERO, s,
            S::ZERO, S::ONE, S::ZERO,
            -s, S::ZERO, c,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation_z<A: Into<Rad<S>>>(theta: A) -> Mat3<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();

        Mat3::new(
            c, s, S::ZERO,
            -s, c, S::ZERO,
            S::ZERO, S::ZERO, S::ONE,
        )
    }
}

impl<S: Num> Matrix for Mat3<S> {
    type Row = Vec3<S>;
    type Column = Vec3<S>;
    type Transpose = Mat3<S>;

    #[inline]
    fn row(&self, r: usize) -> Vec3<S> {
        match r {
            0 => Vec3::new(self.x.x, self.y.x, self.z.x),
            1 => Vec3::new(self.x.y, self.y.y, self.z.y),
            2 => Vec3::new(self.x.z, self.y.z, self.z.z),
            _ => panic!("Invalid row index"),
        }
    }

    #[inline]
    fn column(&self, r: usize) -> Vec3<S> {
        match r {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid column index"),
        }
    }

    #[rustfmt::skip]
    fn transpose(&self) -> Mat3<S> {
        Mat3::new(
            self.x.x, self.y.x, self.z.x,
            self.x.y, self.y.y, self.z.y,
            self.x.z, self.y.z, self.z.z,
        )
    }
}

impl_operator!(<S: Float> Mul<Mat3<S>> for Mat3<S> {
    fn mul(lhs, rhs) -> Mat3<S> {
        Mat3::new(
            lhs.row(0).dot(rhs.column(0)), lhs.row(1).dot(rhs.column(0)), lhs.row(2).dot(rhs.column(0)),
            lhs.row(0).dot(rhs.column(1)), lhs.row(1).dot(rhs.column(1)), lhs.row(2).dot(rhs.column(1)),
            lhs.row(0).dot(rhs.column(2)), lhs.row(1).dot(rhs.column(2)), lhs.row(2).dot(rhs.column(2)),
        )
    }
});

impl<S: Zero> Zero for Mat4<S> {
    const ZERO: Mat4<S> = Mat4::from_cols(Vec4::ZERO, Vec4::ZERO, Vec4::ZERO, Vec4::ZERO);
}

impl<S: Zero + One> Mat4<S> {
    pub const IDENTITY: Mat4<S> = Mat4::from_cols(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W);
}

impl<S> Mat4<S> {
    #[inline]
    #[rustfmt::skip]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        c0r0: S, c0r1: S, c0r2: S, c0r3: S,
        c1r0: S, c1r1: S, c1r2: S, c1r3: S,
        c2r0: S, c2r1: S, c2r2: S, c2r3: S,
        c3r0: S, c3r1: S, c3r2: S, c3r3: S,
    ) -> Mat4<S>  {
        Mat4::from_cols(
            Vec4::new(c0r0, c0r1, c0r2, c0r3),
            Vec4::new(c1r0, c1r1, c1r2, c1r3),
            Vec4::new(c2r0, c2r1, c2r2, c2r3),
            Vec4::new(c3r0, c3r1, c3r2, c3r3),
        )
    }

    #[inline]
    pub const fn from_cols(x: Vec4<S>, y: Vec4<S>, z: Vec4<S>, w: Vec4<S>) -> Mat4<S> {
        Mat4 { x, y, z, w }
    }
}

impl<S: Float> Mat4<S> {
    #[inline]
    #[rustfmt::skip]
    pub fn from_translation(v: Vec3<S>) -> Mat4<S> {
        Mat4::new(
            S::ONE, S::ZERO, S::ZERO, S::ZERO,
            S::ZERO, S::ONE, S::ZERO, S::ZERO,
            S::ZERO, S::ZERO, S::ONE, S::ZERO,
            v.x, v.y, v.z, S::ONE,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_scale(value: S) -> Mat4<S> {
        Mat4::new(
            value, S::ZERO, S::ZERO, S::ZERO,
            S::ZERO, value, S::ZERO, S::ZERO,
            S::ZERO, S::ZERO, value, S::ZERO,
            S::ZERO, S::ZERO, S::ZERO, S::ONE,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_nonuniform_scale(x: S, y: S, z: S) -> Mat4<S> {
        Mat4::new(
            x, S::ZERO, S::ZERO, S::ZERO,
            S::ZERO, y, S::ZERO, S::ZERO,
            S::ZERO, S::ZERO, z, S::ZERO,
            S::ZERO, S::ZERO, S::ZERO, S::ONE,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation<A: Into<Rad<S>>>(axis: Vec3<S>, theta: A) -> Mat4<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();
        let mc = S::ONE - c;

        Mat4::new(
            mc * axis.x * axis.x + c,
            mc * axis.x * axis.y + s * axis.z,
            mc * axis.x * axis.z - s * axis.y,
            S::ZERO,

            mc * axis.x * axis.y - s * axis.z,
            mc * axis.y * axis.y + c,
            mc * axis.y * axis.z + s * axis.x,
            S::ZERO,

            mc * axis.x * axis.z + s * axis.y,
            mc * axis.y * axis.z - s * axis.x,
            mc * axis.z * axis.z + c,
            S::ZERO,

            S::ZERO, S::ZERO, S::ZERO, S::ONE,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation_x<A: Into<Rad<S>>>(theta: A) -> Mat4<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();

        Mat4::new(
            S::ONE, S::ZERO, S::ZERO, S::ZERO,
            S::ZERO, c, s, S::ZERO,
            S::ZERO, -s, c, S::ZERO,
            S::ZERO, S::ZERO, S::ZERO, S::ONE,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation_y<A: Into<Rad<S>>>(theta: A) -> Mat4<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();

        Mat4::new(
            c, S::ZERO, s,S::ZERO,
            S::ZERO, S::ONE, S::ZERO, S::ZERO,
            -s, S::ZERO, c, S::ZERO,
            S::ZERO, S::ZERO, S::ZERO, S::ONE,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation_z<A: Into<Rad<S>>>(theta: A) -> Mat4<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();

        Mat4::new(
            c, s, S::ZERO, S::ZERO,
            -s, c, S::ZERO, S::ZERO,
            S::ZERO, S::ZERO, S::ONE, S::ZERO,
            S::ZERO, S::ZERO, S::ZERO, S::ONE,
        )
    }

    #[rustfmt::skip]
    pub fn look_at(eye: Vec3<S>, center: Vec3<S>, up: Vec3<S>) -> Mat4<S> {
        let f = (center - eye).normalize();
        let s = f.cross(up.normalize());
        let u = s.normalize().cross(f);
        let m = Mat4::new(
            s.x, u.x, -f.x, S::ZERO,
            s.y, u.y, -f.y, S::ZERO,
            s.z, u.z, -f.z, S::ZERO,
            S::ZERO, S::ZERO, S::ZERO, S::ONE,
        );
        m * Mat4::from_translation(-eye)
    }
}

impl<S: Num> Matrix for Mat4<S> {
    type Row = Vec4<S>;
    type Column = Vec4<S>;
    type Transpose = Mat4<S>;

    #[inline]
    fn row(&self, r: usize) -> Vec4<S> {
        match r {
            0 => Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x),
            1 => Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y),
            2 => Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z),
            3 => Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w),
            _ => panic!("Invalid row index"),
        }
    }

    #[inline]
    fn column(&self, r: usize) -> Vec4<S> {
        match r {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => panic!("Invalid column index"),
        }
    }

    #[rustfmt::skip]
    fn transpose(&self) -> Mat4<S> {
        Mat4::new(
            self.x.x, self.y.x, self.z.x, self.w.x,
            self.x.y, self.y.y, self.z.y, self.w.y,
            self.x.z, self.y.z, self.z.z, self.w.z,
            self.x.w, self.y.w, self.z.w, self.w.w,
        )
    }
}

impl_operator!(<S: Float> Mul<Mat4<S>> for Mat4<S> {
    fn mul(lhs, rhs) -> Mat4<S> {
        Mat4::new(
            lhs.row(0).dot(rhs.column(0)), lhs.row(1).dot(rhs.column(0)), lhs.row(2).dot(rhs.column(0)), lhs.row(3).dot(rhs.column(0)),
            lhs.row(0).dot(rhs.column(1)), lhs.row(1).dot(rhs.column(1)), lhs.row(2).dot(rhs.column(1)), lhs.row(3).dot(rhs.column(1)),
            lhs.row(0).dot(rhs.column(2)), lhs.row(1).dot(rhs.column(2)), lhs.row(2).dot(rhs.column(2)), lhs.row(3).dot(rhs.column(2)),
            lhs.row(0).dot(rhs.column(3)), lhs.row(1).dot(rhs.column(3)), lhs.row(2).dot(rhs.column(3)), lhs.row(3).dot(rhs.column(3)),
        )
    }
});

macro_rules! impl_matrix {
    ($MatN:ident, $VecN:ident { $($field:ident : $row_index:expr),+ }) => {
        impl_operator!(<S: Float> Neg for $MatN<S> {
            fn neg(matrix) -> $MatN<S> { $MatN { $($field: -matrix.$field),+ } }
        });
        impl_operator!(<S: Float> Mul<S> for $MatN<S> {
            fn mul(matrix, scalar) -> $MatN<S> { $MatN { $($field: matrix.$field * scalar),+ } }
        });
        impl_operator!(<S: Float> Div<S> for $MatN<S> {
            fn div(matrix, scalar) -> $MatN<S> { $MatN { $($field: matrix.$field / scalar),+ } }
        });

        impl_operator!(<S: Float> Add<$MatN<S> > for $MatN<S> {
            fn add(lhs, rhs) -> $MatN<S> { $MatN { $($field: lhs.$field + rhs.$field),+ } }
        });
        impl_operator!(<S: Float> Sub<$MatN<S> > for $MatN<S> {
            fn sub(lhs, rhs) -> $MatN<S> { $MatN { $($field: lhs.$field - rhs.$field),+ } }
        });

        impl_scalar_ops!($MatN<usize> { $($field),+ });
        impl_scalar_ops!($MatN<u8> { $($field),+ });
        impl_scalar_ops!($MatN<u16> { $($field),+ });
        impl_scalar_ops!($MatN<u32> { $($field),+ });
        impl_scalar_ops!($MatN<u64> { $($field),+ });
        impl_scalar_ops!($MatN<isize> { $($field),+ });
        impl_scalar_ops!($MatN<i8> { $($field),+ });
        impl_scalar_ops!($MatN<i16> { $($field),+ });
        impl_scalar_ops!($MatN<i32> { $($field),+ });
        impl_scalar_ops!($MatN<i64> { $($field),+ });
        impl_scalar_ops!($MatN<f32> { $($field),+ });
        impl_scalar_ops!($MatN<f64> { $($field),+ });
    }
}

macro_rules! impl_scalar_ops {
    ($MatN:ident<$S:ident> { $($field:ident),+ }) => {
        impl_operator!(Mul<$MatN<$S>> for $S {
            fn mul(scalar, matrix) -> $MatN<$S> { $MatN { $($field: scalar * matrix.$field),+ } }
        });
        impl_operator!(Div<$MatN<$S>> for $S {
            fn div(scalar, matrix) -> $MatN<$S> { $MatN { $($field: scalar / matrix.$field),+ } }
        });
    };
}

impl_matrix!(Mat2, Vec2 { x: 0, y: 1 });
impl_matrix!(Mat3, Vec3 { x: 0, y: 1, z: 2 });
#[rustfmt::skip]
impl_matrix!(Mat4, Vec4 { x: 0, y: 1, z: 2, w: 3});

impl<S: Float> Mul<Vec2<S>> for Mat2<S> {
    type Output = Vec2<S>;

    fn mul(self, rhs: Vec2<S>) -> Self::Output {
        Vec2::new(self.row(0).dot(rhs), self.row(1).dot(rhs))
    }
}

impl<'a, S: Float> Mul<&'a Vec2<S>> for Mat2<S> {
    type Output = Vec2<S>;

    fn mul(self, rhs: &'a Vec2<S>) -> Self::Output {
        Vec2::new(self.row(0).dot(*rhs), self.row(1).dot(*rhs))
    }
}

impl<S: Float> Mul<Vec3<S>> for Mat3<S> {
    type Output = Vec3<S>;

    fn mul(self, rhs: Vec3<S>) -> Self::Output {
        Vec3::new(
            self.row(0).dot(rhs),
            self.row(1).dot(rhs),
            self.row(2).dot(rhs),
        )
    }
}

impl<'a, S: Float> Mul<&'a Vec3<S>> for Mat3<S> {
    type Output = Vec3<S>;

    fn mul(self, rhs: &'a Vec3<S>) -> Self::Output {
        Vec3::new(
            self.row(0).dot(*rhs),
            self.row(1).dot(*rhs),
            self.row(2).dot(*rhs),
        )
    }
}

impl<S: Float> Mul<Vec4<S>> for Mat4<S> {
    type Output = Vec4<S>;

    fn mul(self, rhs: Vec4<S>) -> Self::Output {
        Vec4::new(
            self.row(0).dot(rhs),
            self.row(1).dot(rhs),
            self.row(2).dot(rhs),
            self.row(3).dot(rhs),
        )
    }
}

impl<'a, S: Float> Mul<&'a Vec4<S>> for Mat4<S> {
    type Output = Vec4<S>;

    fn mul(self, rhs: &'a Vec4<S>) -> Self::Output {
        Vec4::new(
            self.row(0).dot(*rhs),
            self.row(1).dot(*rhs),
            self.row(2).dot(*rhs),
            self.row(3).dot(*rhs),
        )
    }
}
