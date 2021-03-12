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
impl_zero!(u8);
impl_zero!(u16);
impl_zero!(u32);
impl_zero!(u64);
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
impl_one!(u8);
impl_one!(u16);
impl_one!(u32);
impl_one!(u64);
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

impl Num for i8 {}
impl Num for i16 {}
impl Num for i32 {}
impl Num for i64 {}
impl Num for f32 {}
impl Num for f64 {}

pub trait Float: Num {
    fn sqrt(self) -> Self;
}

impl Float for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Float for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
