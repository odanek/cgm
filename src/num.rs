use std::ops::*;

pub trait Zero: Sized + Add<Self, Output = Self> {
    const ZERO: Self;
}

macro_rules! impl_zero {
    ($t:ty, $val:expr) => {
        impl Zero for $t {
            const ZERO: Self = $val;
        }
    };
}

impl_zero!(i8, 0i8);
impl_zero!(i16, 0i16);
impl_zero!(i32, 0i32);
impl_zero!(i64, 0i64);
impl_zero!(isize, 0isize);
impl_zero!(u8, 0u8);
impl_zero!(u16, 0u16);
impl_zero!(u32, 0u32);
impl_zero!(u64, 0u64);
impl_zero!(usize, 0usize);
impl_zero!(f32, 0f32);
impl_zero!(f64, 0f64);

pub trait One: Sized + Mul<Self, Output = Self> {
    const ONE: Self;
}

macro_rules! impl_one {
    ($t:ty, $val:expr) => {
        impl One for $t {
            const ONE: Self = 1 as $t;
        }
    };
}

impl_one!(i8, 1i8);
impl_one!(i16, 1i16);
impl_one!(i32, 1i32);
impl_one!(i64, 1i64);
impl_one!(isize, 1isize);
impl_one!(u8, 1u8);
impl_one!(u16, 1u16);
impl_one!(u32, 1u32);
impl_one!(u64, 1u64);
impl_one!(usize, 1usize);
impl_one!(f32, 1f32);
impl_one!(f64, 1f64);

pub trait NumOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
{
}

impl<T, Rhs, Output> NumOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
        + Rem<Rhs, Output = Output>
{
}

pub trait NumAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{
}

impl<T, Rhs> NumAssignOps<Rhs> for T where
    T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{
}

pub trait Num: Copy + Zero + One + PartialOrd + NumOps + NumAssignOps {}

impl Num for u8 {}
impl Num for i8 {}
impl Num for u16 {}
impl Num for i16 {}
impl Num for u32 {}
impl Num for i32 {}
impl Num for u64 {}
impl Num for i64 {}
impl Num for usize {}
impl Num for isize {}
impl Num for f32 {}
impl Num for f64 {}

pub trait Signed: Num + Neg<Output = Self> {
    fn abs(self) -> Self;
    fn signum(self) -> Self;
}

macro_rules! impl_signed {
    ($t:ty) => {
        impl Signed for $t {
            #[inline]
            fn abs(self) -> Self {
                self.abs()
            }

            #[inline]
            fn signum(self) -> Self {
                self.signum()
            }
        }
    };
}

impl_signed!(i8);
impl_signed!(i16);
impl_signed!(i32);
impl_signed!(i64);
impl_signed!(f32);
impl_signed!(f64);

pub trait Float: Signed {
    const HALF: Self;
    const RAD_FULL_TURN: Self;
    const RAD_HALF_TURN: Self;
    const DEG_FULL_TURN: Self;
    const DEG_HALF_TURN: Self;
    const DEG_RAD_RATIO: Self;

    fn sqrt(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    fn recip(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
}

// TODO: Macro
impl Float for f32 {
    const HALF: Self = 0.5;
    const RAD_FULL_TURN: Self = 2.0 * std::f32::consts::PI;
    const RAD_HALF_TURN: Self = std::f32::consts::PI;
    const DEG_FULL_TURN: Self = 360.0;
    const DEG_HALF_TURN: Self = 180.0;
    const DEG_RAD_RATIO: Self = 180.0 / std::f32::consts::PI;

    #[inline]
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    #[inline]
    fn round(self) -> Self {
        self.round()
    }
    #[inline]
    fn trunc(self) -> Self {
        self.trunc()
    }
    #[inline]
    fn fract(self) -> Self {
        self.fract()
    }
    #[inline]
    fn recip(self) -> Self {
        self.recip()
    }
    #[inline]
    fn sin(self) -> Self {
        self.sin()
    }
    #[inline]
    fn cos(self) -> Self {
        self.cos()
    }
    #[inline]
    fn tan(self) -> Self {
        self.tan()
    }
    #[inline]
    fn asin(self) -> Self {
        self.asin()
    }
    #[inline]
    fn acos(self) -> Self {
        self.acos()
    }
    #[inline]
    fn atan(self) -> Self {
        self.atan()
    }
    #[inline]
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }
    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }
    #[inline]
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
    #[inline]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
}

impl Float for f64 {
    const HALF: Self = 0.5;
    const RAD_FULL_TURN: Self = 2.0 * std::f64::consts::PI;
    const RAD_HALF_TURN: Self = std::f64::consts::PI;
    const DEG_FULL_TURN: Self = 360.0;
    const DEG_HALF_TURN: Self = 180.0;
    const DEG_RAD_RATIO: Self = 180.0 / std::f64::consts::PI;

    #[inline]
    fn sqrt(self) -> Self {
        self.sqrt()
    }
    #[inline]
    fn round(self) -> Self {
        self.round()
    }
    #[inline]
    fn trunc(self) -> Self {
        self.trunc()
    }
    #[inline]
    fn fract(self) -> Self {
        self.fract()
    }
    #[inline]
    fn recip(self) -> Self {
        self.recip()
    }
    #[inline]
    fn sin(self) -> Self {
        self.sin()
    }
    #[inline]
    fn cos(self) -> Self {
        self.cos()
    }
    #[inline]
    fn tan(self) -> Self {
        self.tan()
    }
    #[inline]
    fn asin(self) -> Self {
        self.asin()
    }
    #[inline]
    fn acos(self) -> Self {
        self.acos()
    }
    #[inline]
    fn atan(self) -> Self {
        self.atan()
    }
    #[inline]
    fn atan2(self, other: Self) -> Self {
        self.atan2(other)
    }
    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        self.sin_cos()
    }
    #[inline]
    fn min(self, other: Self) -> Self {
        self.min(other)
    }
    #[inline]
    fn max(self, other: Self) -> Self {
        self.max(other)
    }
}
