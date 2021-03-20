use std::fmt::Debug;
use std::ops::*;

pub trait Zero {
    const ZERO: Self;
}

macro_rules! impl_zero {
    ($t:ty) => {
        impl Zero for $t {
            const ZERO: Self = 0 as $t;
        }
    };
}

impl_zero!(i8);
impl_zero!(i16);
impl_zero!(i32);
impl_zero!(i64);
impl_zero!(isize);
impl_zero!(u8);
impl_zero!(u16);
impl_zero!(u32);
impl_zero!(u64);
impl_zero!(usize);
impl_zero!(f32);
impl_zero!(f64);

pub trait One {
    const ONE: Self;
}

macro_rules! impl_one {
    ($t:ty) => {
        impl One for $t {
            const ONE: Self = 1 as $t;
        }
    };
}

impl_one!(i8);
impl_one!(i16);
impl_one!(i32);
impl_one!(i64);
impl_one!(isize);
impl_one!(u8);
impl_one!(u16);
impl_one!(u32);
impl_one!(u64);
impl_one!(usize);
impl_one!(f32);
impl_one!(f64);

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

pub trait Num: Copy + Clone + Debug + Zero + One + NumOps + PartialOrd {}

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

impl Float for f32 {
    const RAD_FULL_TURN: Self = 2.0 * std::f32::consts::PI;
    const RAD_HALF_TURN: Self = std::f32::consts::PI;
    const DEG_FULL_TURN: Self = 360.0;
    const DEG_HALF_TURN: Self = 180.0;    

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
