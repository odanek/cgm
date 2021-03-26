use std::fmt::Debug;
use std::ops::*;

pub trait Zero {
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

pub trait One {
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

pub trait Num:
    Copy
    + Clone
    + Debug
    + Zero
    + One
    + PartialOrd
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>
{
}

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

pub trait SignedNum: Num + Neg<Output = Self> {
    fn abs(self) -> Self;
}

// TODO: Macro
impl SignedNum for i8 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }
}
impl SignedNum for i16 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }
}
impl SignedNum for i32 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }
}
impl SignedNum for i64 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }
}
impl SignedNum for f32 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }
}
impl SignedNum for f64 {
    #[inline]
    fn abs(self) -> Self {
        self.abs()
    }
}

pub trait Float: SignedNum {
    const RAD_FULL_TURN: Self;
    const RAD_HALF_TURN: Self;
    const DEG_FULL_TURN: Self;
    const DEG_HALF_TURN: Self;
    const DEG_RAD_RATIO: Self;

    fn sqrt(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
}

// TODO: Macro
impl Float for f32 {
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
}

impl Float for f64 {
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
}
