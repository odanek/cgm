use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::{
    Angle, ElementWise, Float, InnerSpace, MetricSpace, Num, One, Rad, Signed, VectorSpace, Zero,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Vec1<S> {
    pub x: S,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Vec2<S> {
    pub x: S,
    pub y: S,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Vec3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Vec4<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    pub w: S,
}

pub trait Vector {
    type Element: Copy;

    fn length() -> usize;
    fn from_value(scalar: Self::Element) -> Self;
    fn sum(self) -> Self::Element
    where
        Self::Element: Add<Output = Self::Element>;
    fn product(self) -> Self::Element
    where
        Self::Element: Mul<Output = Self::Element>;
}

macro_rules! impl_vector {
    ($VecN:ident { $($field:ident),+ }, $n:expr) => {
        impl<S> $VecN<S> {
            #[inline]
            pub const fn new($($field: S),+) -> Self {
                Self { $($field),+ }
            }
        }

        impl<S: Float> $VecN<S> {
            #[inline]
            pub fn abs(&self) -> Self {
                Self { $($field: self.$field.abs()),+ }
            }
            #[inline]
            pub fn round(&self) -> Self {
                Self { $($field: self.$field.round()),+ }
            }
            #[inline]
            pub fn trunc(&self) -> Self {
                Self { $($field: self.$field.trunc()),+ }
            }
            #[inline]
            pub fn fract(&self) -> Self {
                Self { $($field: self.$field.fract()),+ }
            }
            #[inline]
            pub fn recip(&self) -> Self {
                Self { $($field: self.$field.recip()),+ }
            }
        }

        impl<S: Num> Zero for $VecN<S> {
            const ZERO: $VecN<S> = $VecN { $($field: S::ZERO),+ };
        }

        impl<S: Num> VectorSpace for $VecN<S> {
            type Scalar = S;
        }

        impl<S: Num> MetricSpace for $VecN<S> {
            type Metric = S;

            #[inline]
            fn distance2(self, other: Self) -> S {
                (other - self).magnitude2()
            }
        }

        impl_operator!(<S: Num> Add<$VecN<S>> for $VecN<S> {
            fn add(lhs, rhs) -> $VecN<S> { $VecN { $($field: lhs.$field + rhs.$field),+ } }
        });
        impl_assignment_operator!(<S: Num> AddAssign<$VecN<S> > for $VecN<S> {
            fn add_assign(&mut self, other) { $(self.$field += other.$field);+ }
        });

        impl_operator!(<S: Num> Sub<$VecN<S>> for $VecN<S> {
            fn sub(lhs, rhs) -> $VecN<S> { $VecN { $($field: lhs.$field - rhs.$field),+ } }
        });
        impl_assignment_operator!(<S: Num> SubAssign<$VecN<S> > for $VecN<S> {
            fn sub_assign(&mut self, other) { $(self.$field -= other.$field);+ }
        });

        impl_operator!(<S: Num> Mul<S> for $VecN<S> {
            fn mul(lhs, rhs) -> $VecN<S> { $VecN { $($field: lhs.$field * rhs),+ } }
        });
        impl_assignment_operator!(<S: Num> MulAssign<S> for $VecN<S> {
            fn mul_assign(&mut self, scalar) { $(self.$field *= scalar);+ }
        });

        impl_operator!(<S: Num> Div<S> for $VecN<S> {
            fn div(lhs, rhs) -> $VecN<S> { $VecN { $($field: lhs.$field / rhs),+ } }
        });
        impl_assignment_operator!(<S: Num> DivAssign<S> for $VecN<S> {
            fn div_assign(&mut self, scalar) { $(self.$field /= scalar);+ }
        });

        impl_operator!(<S: Num> Rem<S> for $VecN<S> {
            fn rem(lhs, scalar) -> $VecN<S> { $VecN { $($field: lhs.$field % scalar),+ } }
        });
        impl_assignment_operator!(<S: Num> RemAssign<S> for $VecN<S> {
            fn rem_assign(&mut self, scalar) { $(self.$field %= scalar);+ }
        });

        impl_operator!(<S: Signed> Neg for $VecN<S> {
            fn neg(vec) -> $VecN<S> { $VecN { $($field: -vec.$field),+ } }
        });

        impl_scalar_ops!($VecN<usize> { $($field),+ });
        impl_scalar_ops!($VecN<u8> { $($field),+ });
        impl_scalar_ops!($VecN<u16> { $($field),+ });
        impl_scalar_ops!($VecN<u32> { $($field),+ });
        impl_scalar_ops!($VecN<u64> { $($field),+ });
        impl_scalar_ops!($VecN<isize> { $($field),+ });
        impl_scalar_ops!($VecN<i8> { $($field),+ });
        impl_scalar_ops!($VecN<i16> { $($field),+ });
        impl_scalar_ops!($VecN<i32> { $($field),+ });
        impl_scalar_ops!($VecN<i64> { $($field),+ });
        impl_scalar_ops!($VecN<f32> { $($field),+ });
        impl_scalar_ops!($VecN<f64> { $($field),+ });

        impl<S: Num> ElementWise for $VecN<S> {
            #[inline] fn add_element_wise(self, rhs: $VecN<S>) -> $VecN<S> { $VecN::new($(self.$field + rhs.$field),+) }
            #[inline] fn sub_element_wise(self, rhs: $VecN<S>) -> $VecN<S> { $VecN::new($(self.$field - rhs.$field),+) }
            #[inline] fn mul_element_wise(self, rhs: $VecN<S>) -> $VecN<S> { $VecN::new($(self.$field * rhs.$field),+) }
            #[inline] fn div_element_wise(self, rhs: $VecN<S>) -> $VecN<S> { $VecN::new($(self.$field / rhs.$field),+) }
            #[inline] fn rem_element_wise(self, rhs: $VecN<S>) -> $VecN<S> { $VecN::new($(self.$field % rhs.$field),+) }

            #[inline] fn add_assign_element_wise(&mut self, rhs: $VecN<S>) { $(self.$field += rhs.$field);+ }
            #[inline] fn sub_assign_element_wise(&mut self, rhs: $VecN<S>) { $(self.$field -= rhs.$field);+ }
            #[inline] fn mul_assign_element_wise(&mut self, rhs: $VecN<S>) { $(self.$field *= rhs.$field);+ }
            #[inline] fn div_assign_element_wise(&mut self, rhs: $VecN<S>) { $(self.$field /= rhs.$field);+ }
            #[inline] fn rem_assign_element_wise(&mut self, rhs: $VecN<S>) { $(self.$field %= rhs.$field);+ }
        }

        impl<S: Num> ElementWise<S> for $VecN<S> {
            #[inline] fn add_element_wise(self, rhs: S) -> $VecN<S> { $VecN::new($(self.$field + rhs),+) }
            #[inline] fn sub_element_wise(self, rhs: S) -> $VecN<S> { $VecN::new($(self.$field - rhs),+) }
            #[inline] fn mul_element_wise(self, rhs: S) -> $VecN<S> { $VecN::new($(self.$field * rhs),+) }
            #[inline] fn div_element_wise(self, rhs: S) -> $VecN<S> { $VecN::new($(self.$field / rhs),+) }
            #[inline] fn rem_element_wise(self, rhs: S) -> $VecN<S> { $VecN::new($(self.$field % rhs),+) }

            #[inline] fn add_assign_element_wise(&mut self, rhs: S) { $(self.$field += rhs);+ }
            #[inline] fn sub_assign_element_wise(&mut self, rhs: S) { $(self.$field -= rhs);+ }
            #[inline] fn mul_assign_element_wise(&mut self, rhs: S) { $(self.$field *= rhs);+ }
            #[inline] fn div_assign_element_wise(&mut self, rhs: S) { $(self.$field /= rhs);+ }
            #[inline] fn rem_assign_element_wise(&mut self, rhs: S) { $(self.$field %= rhs);+ }
        }

        impl<S: Num> $VecN<S> { // TODO Make this a trait and add min/max with scalar?
            #[inline] pub fn min_element_wise(self, rhs: $VecN<S>) -> $VecN<S> { $VecN::new($(self.$field.min(rhs.$field)),+) }
            #[inline] pub fn max_element_wise(self, rhs: $VecN<S>) -> $VecN<S> { $VecN::new($(self.$field.max(rhs.$field)),+) }
        }
    }
}

macro_rules! impl_scalar_ops {
    ($VecN:ident<$S:ident> { $($field:ident),+ }) => {
        impl_operator!(Mul<$VecN<$S>> for $S {
            fn mul(scalar, vector) -> $VecN<$S> { $VecN { $($field: scalar * vector.$field),+ } }
        });
        impl_operator!(Div<$VecN<$S>> for $S {
            fn div(scalar, vector) -> $VecN<$S> { $VecN { $($field: scalar / vector.$field),+ } }
        });
        impl_operator!(Rem<$VecN<$S>> for $S {
            fn rem(scalar, vector) -> $VecN<$S> { $VecN { $($field: scalar % vector.$field),+ } }
        });
    };
}

impl_vector!(Vec1 { x }, 1);
impl_vector!(Vec2 { x, y }, 2);
impl_vector!(Vec3 { x, y, z }, 3);
impl_vector!(Vec4 { x, y, z, w }, 4);

impl_fixed_array_conversions!(Vec1<S> { x: 0 }, 1);
impl_fixed_array_conversions!(Vec2<S> { x: 0, y: 1 }, 2);
impl_fixed_array_conversions!(Vec3<S> { x: 0, y: 1, z: 2 }, 3);
impl_fixed_array_conversions!(Vec4<S> { x: 0, y: 1, z: 2, w: 3 }, 4);

impl_tuple_conversions!(Vec1<S> { x }, (S,));
impl_tuple_conversions!(Vec2<S> { x, y }, (S, S));
impl_tuple_conversions!(Vec3<S> { x, y, z }, (S, S, S));
impl_tuple_conversions!(Vec4<S> { x, y, z, w }, (S, S, S, S));

impl<S: Num> InnerSpace for Vec1<S> {
    #[inline]
    fn dot(self, other: Vec1<S>) -> S {
        self.x * other.x
    }
}

impl<S: Num> Vector for Vec1<S> {
    type Element = S;

    #[inline]
    fn length() -> usize {
        1
    }

    #[inline]
    fn from_value(scalar: Self::Element) -> Self {
        Vec1 { x: scalar }
    }

    #[inline]
    fn sum(self) -> S {
        self.x
    }

    #[inline]
    fn product(self) -> S {
        self.x
    }
}

impl<S: Num> InnerSpace for Vec2<S> {
    #[inline]
    fn dot(self, other: Vec2<S>) -> S {
        (self.x * other.x) + (self.y * other.y)
    }

    #[inline]
    fn angle(self, other: Vec2<S>) -> Rad<S>
    where
        S: Float,
    {
        Rad::atan2(Self::perp_dot(self, other), Self::dot(self, other))
    }
}

impl<S: Num> Vector for Vec2<S> {
    type Element = S;

    #[inline]
    fn length() -> usize {
        2
    }

    #[inline]
    fn from_value(scalar: Self::Element) -> Self {
        Vec2 {
            x: scalar,
            y: scalar,
        }
    }

    #[inline]
    fn sum(self) -> S {
        self.x + self.y
    }

    #[inline]
    fn product(self) -> S {
        self.x * self.y
    }
}

impl<S: Num> InnerSpace for Vec3<S> {
    #[inline]
    fn dot(self, other: Vec3<S>) -> S {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    #[inline]
    fn angle(self, other: Vec3<S>) -> Rad<S>
    where
        S: Float,
    {
        Rad::atan2(self.cross(other).magnitude(), Self::dot(self, other))
    }
}

impl<S: Num> Vector for Vec3<S> {
    type Element = S;

    #[inline]
    fn length() -> usize {
        3
    }

    #[inline]
    fn from_value(scalar: Self::Element) -> Self {
        Vec3 {
            x: scalar,
            y: scalar,
            z: scalar,
        }
    }

    #[inline]
    fn sum(self) -> S {
        self.x + self.y + self.z
    }

    #[inline]
    fn product(self) -> S {
        self.x * self.y * self.z
    }
}

impl<S: Num> InnerSpace for Vec4<S> {
    #[inline]
    fn dot(self, other: Vec4<S>) -> S {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z) + (self.w * other.w)
    }
}

impl<S: Num> Vector for Vec4<S> {
    type Element = S;

    #[inline]
    fn length() -> usize {
        4
    }

    #[inline]
    fn from_value(scalar: Self::Element) -> Self {
        Vec4 {
            x: scalar,
            y: scalar,
            z: scalar,
            w: scalar,
        }
    }

    #[inline]
    fn sum(self) -> S {
        self.x + self.y + self.z + self.w
    }

    #[inline]
    fn product(self) -> S {
        self.x * self.y * self.z * self.w
    }
}

impl<S: Num> Vec1<S> {
    #[inline]
    pub fn extend(self, y: S) -> Vec2<S> {
        Vec2 { x: self.x, y }
    }

    #[inline]
    pub fn from_slice(slice: &[S; 1]) -> Self {
        Self { x: slice[0] }
    }
}

impl<S: One> Vec1<S> {
    pub const X: Vec1<S> = Vec1 { x: S::ONE };
}

impl<S: Num> Vec2<S> {
    #[inline]
    pub fn extend(self, z: S) -> Vec3<S> {
        Vec3 {
            x: self.x,
            y: self.y,
            z,
        }
    }

    #[inline]
    pub fn truncate(self) -> Vec1<S> {
        Vec1 { x: self.x }
    }

    #[inline]
    pub fn from_slice(slice: &[S; 2]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
        }
    }
}

impl<S: Zero + One> Vec2<S> {
    pub const X: Vec2<S> = Vec2 {
        x: S::ONE,
        y: S::ZERO,
    };
    pub const Y: Vec2<S> = Vec2 {
        x: S::ZERO,
        y: S::ONE,
    };
}

impl<S: Num> Vec2<S> {
    #[inline]
    pub fn perp_dot(self, other: Vec2<S>) -> S {
        (self.x * other.y) - (self.y * other.x)
    }
}

impl<S: Num> Vec3<S> {
    #[inline]
    pub fn extend(self, w: S) -> Vec4<S> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }

    #[inline]
    pub fn truncate(self) -> Vec2<S> {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    #[inline]
    pub fn from_slice(slice: &[S; 3]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }
}

impl<S: Zero + One> Vec3<S> {
    pub const X: Vec3<S> = Vec3 {
        x: S::ONE,
        y: S::ZERO,
        z: S::ZERO,
    };
    pub const Y: Vec3<S> = Vec3 {
        x: S::ZERO,
        y: S::ONE,
        z: S::ZERO,
    };
    pub const Z: Vec3<S> = Vec3 {
        x: S::ZERO,
        y: S::ZERO,
        z: S::ONE,
    };
}

impl<S: Num> Vec3<S> {
    #[inline]
    pub fn cross(self, other: Vec3<S>) -> Vec3<S> {
        Vec3 {
            x: (self.y * other.z) - (self.z * other.y),
            y: (self.z * other.x) - (self.x * other.z),
            z: (self.x * other.y) - (self.y * other.x),
        }
    }

    #[inline]
    pub fn from_homogeneous(v: Vec4<S>) -> Vec3<S> {
        Vec3 {
            x: v.x / v.w,
            y: v.y / v.w,
            z: v.z / v.w,
        }
    }

    #[inline]
    pub fn to_homogeneous(self) -> Vec4<S> {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w: S::ONE,
        }
    }
}

impl<S: Num> Vec4<S> {
    #[inline]
    pub fn truncate(self) -> Vec3<S> {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    #[inline]
    pub fn truncate_n(&self, n: isize) -> Vec3<S> {
        match n {
            0 => Vec3::new(self.y, self.z, self.w),
            1 => Vec3::new(self.x, self.z, self.w),
            2 => Vec3::new(self.x, self.y, self.w),
            3 => Vec3::new(self.x, self.y, self.z),
            _ => panic!("{:?} is out of range", n),
        }
    }

    #[inline]
    pub fn from_slice(slice: &[S; 4]) -> Self {
        Self {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }
}

impl<S: Zero + One> Vec4<S> {
    pub const X: Vec4<S> = Vec4 {
        x: S::ONE,
        y: S::ZERO,
        z: S::ZERO,
        w: S::ZERO,
    };
    pub const Y: Vec4<S> = Vec4 {
        x: S::ZERO,
        y: S::ONE,
        z: S::ZERO,
        w: S::ZERO,
    };
    pub const Z: Vec4<S> = Vec4 {
        x: S::ZERO,
        y: S::ZERO,
        z: S::ONE,
        w: S::ZERO,
    };
    pub const W: Vec4<S> = Vec4 {
        x: S::ZERO,
        y: S::ZERO,
        z: S::ZERO,
        w: S::ONE,
    };
}

#[cfg(test)]
mod tests {
    mod vec1 {
        use crate::{Vec1, Zero};

        #[test]
        fn test_zero() {
            let v = Vec1::<f32>::ZERO;
            assert_eq!(v.x, 0.0);
        }

        #[test]
        fn test_constructor() {
            let v = Vec1::new(5.0);
            assert_eq!(v.x, 5.0);
        }

        #[test]
        fn test_add() {
            let v = Vec1::new(3.0) + Vec1::new(5.0);
            assert_eq!(v.x, 8.0);
        }

        #[test]
        fn test_sub() {
            let v = Vec1::new(3.0) - Vec1::new(5.0);
            assert_eq!(v.x, -2.0);
        }

        #[test]
        fn test_mul() {
            let a = Vec1::new(3.0) * 2.0;
            assert_eq!(a.x, 6.0);
            let b = 2.0f64 * Vec1::new(3.0); // TODO: Why is the f64 needed?
            assert_eq!(b.x, 6.0);
        }

        #[test]
        fn test_div() {
            let a = Vec1::new(4.0) / 2.0;
            assert_eq!(a.x, 2.0);
            let b = 4.0f64 / Vec1::new(2.0); // TODO: Why is the f64 needed?
            assert_eq!(b.x, 2.0);
        }
    }

    mod vec2 {
        use crate::{Vec2, Zero};

        #[test]
        fn test_zero() {
            let v = Vec2::<f32>::ZERO;
            assert_eq!(v.x, 0.0);
            assert_eq!(v.y, 0.0);
        }

        #[test]
        fn test_constructor() {
            let v = Vec2::new(5.0, 3.0);
            assert_eq!(v.x, 5.0);
            assert_eq!(v.y, 3.0);
        }

        #[test]
        fn test_add() {
            let v = Vec2::new(3.0, 4.0) + Vec2::new(5.0, 6.0);
            assert_eq!(v.x, 8.0);
            assert_eq!(v.y, 10.0);
        }

        #[test]
        fn test_sub() {
            let v = Vec2::new(3.0, 4.0) - Vec2::new(5.0, 1.0);
            assert_eq!(v.x, -2.0);
            assert_eq!(v.y, 3.0);
        }

        #[test]
        fn test_mul() {
            let a = Vec2::new(3.0, 4.0) * 2.0;
            assert_eq!(a.x, 6.0);
            assert_eq!(a.y, 8.0);
            let b = 2.0f64 * Vec2::new(3.0, 4.0); // TODO: Why is the f64 needed?
            assert_eq!(b.x, 6.0);
            assert_eq!(b.y, 8.0);
        }

        #[test]
        fn test_div() {
            let a = Vec2::new(4.0, 6.0) / 2.0;
            assert_eq!(a.x, 2.0);
            assert_eq!(a.y, 3.0);
            let b = 4.0f64 / Vec2::new(2.0, 1.0); // TODO: Why is the f64 needed?
            assert_eq!(b.x, 2.0);
            assert_eq!(b.y, 4.0);
        }
    }

    mod vec3 {
        use crate::{Vec3, Zero};

        #[test]
        fn test_zero() {
            let v = Vec3::<f32>::ZERO;
            assert_eq!(v.x, 0.0);
            assert_eq!(v.y, 0.0);
            assert_eq!(v.z, 0.0);
        }

        #[test]
        fn test_constructor() {
            let v = Vec3::new(5.0, 3.0, 1.0);
            assert_eq!(v.x, 5.0);
            assert_eq!(v.y, 3.0);
            assert_eq!(v.z, 1.0);
        }

        #[test]
        fn test_add() {
            let v = Vec3::new(3.0, 4.0, 1.0) + Vec3::new(5.0, 6.0, 2.0);
            assert_eq!(v.x, 8.0);
            assert_eq!(v.y, 10.0);
            assert_eq!(v.z, 3.0);
        }

        #[test]
        fn test_sub() {
            let v = Vec3::new(3.0, 4.0, 7.0) - Vec3::new(5.0, 1.0, 2.0);
            assert_eq!(v.x, -2.0);
            assert_eq!(v.y, 3.0);
            assert_eq!(v.z, 5.0);
        }

        #[test]
        fn test_mul() {
            let a = Vec3::new(3.0, 4.0, 1.0) * 2.0;
            assert_eq!(a.x, 6.0);
            assert_eq!(a.y, 8.0);
            assert_eq!(a.z, 2.0);
            let b = 2.0f64 * Vec3::new(3.0, 4.0, 1.0);
            assert_eq!(b.x, 6.0);
            assert_eq!(b.y, 8.0);
            assert_eq!(b.z, 2.0);
        }

        #[test]
        fn test_div() {
            let a = Vec3::new(4.0, 6.0, 2.0) / 2.0;
            assert_eq!(a.x, 2.0);
            assert_eq!(a.y, 3.0);
            assert_eq!(a.z, 1.0);
            let b = 4.0f64 / Vec3::new(2.0, 1.0, 4.0);
            assert_eq!(b.x, 2.0);
            assert_eq!(b.y, 4.0);
            assert_eq!(b.z, 1.0);
        }
    }

    mod vec4 {
        use crate::{Vec4, Zero};

        #[test]
        fn test_zero() {
            let v = Vec4::<f32>::ZERO;
            assert_eq!(v.x, 0.0);
            assert_eq!(v.y, 0.0);
            assert_eq!(v.z, 0.0);
            assert_eq!(v.w, 0.0);
        }

        #[test]
        fn test_constructor() {
            let v = Vec4::new(5.0, 3.0, 1.0, 2.0);
            assert_eq!(v.x, 5.0);
            assert_eq!(v.y, 3.0);
            assert_eq!(v.z, 1.0);
            assert_eq!(v.w, 2.0);
        }

        #[test]
        fn test_add() {
            let v = Vec4::new(3.0, 4.0, 1.0, 1.0) + Vec4::new(5.0, 6.0, 2.0, 6.0);
            assert_eq!(v.x, 8.0);
            assert_eq!(v.y, 10.0);
            assert_eq!(v.z, 3.0);
            assert_eq!(v.w, 7.0);
        }

        #[test]
        fn test_sub() {
            let v = Vec4::new(3.0, 4.0, 7.0, 5.0) - Vec4::new(5.0, 1.0, 2.0, 4.0);
            assert_eq!(v.x, -2.0);
            assert_eq!(v.y, 3.0);
            assert_eq!(v.z, 5.0);
            assert_eq!(v.w, 1.0);
        }

        #[test]
        fn test_mul() {
            let a = Vec4::new(3.0, 4.0, 1.0, 2.0) * 2.0;
            assert_eq!(a.x, 6.0);
            assert_eq!(a.y, 8.0);
            assert_eq!(a.z, 2.0);
            assert_eq!(a.w, 4.0);
            let b = 2.0f64 * Vec4::new(3.0, 4.0, 1.0, 2.0);
            assert_eq!(b.x, 6.0);
            assert_eq!(b.y, 8.0);
            assert_eq!(b.z, 2.0);
            assert_eq!(b.w, 4.0);
        }

        #[test]
        fn test_div() {
            let a = Vec4::new(4.0, 6.0, 2.0, 8.0) / 2.0;
            assert_eq!(a.x, 2.0);
            assert_eq!(a.y, 3.0);
            assert_eq!(a.z, 1.0);
            assert_eq!(a.w, 4.0);
            let b = 4.0f64 / Vec4::new(2.0, 1.0, 4.0, 8.0);
            assert_eq!(b.x, 2.0);
            assert_eq!(b.y, 4.0);
            assert_eq!(b.z, 1.0);
            assert_eq!(b.w, 0.5);
        }
    }
}
