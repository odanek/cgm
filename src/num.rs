use std::fmt::Debug;
use std::ops::*;

pub trait Zero {
    const ZERO: Self;
}

// TODO Macro
impl Zero for i8 {
    const ZERO: Self = 0i8;
}

impl Zero for i16 {
    const ZERO: Self = 0i16;
}

impl Zero for i32 {
    const ZERO: Self = 0i32;
}

impl Zero for i64 {
    const ZERO: Self = 0i64;
}

impl Zero for f32 {
    const ZERO: Self = 0f32;
}

impl Zero for f64 {
    const ZERO: Self = 0f64;
}

pub trait One {
    const ONE: Self;
}

// TODO Macro
impl One for i8 {
    const ONE: Self = 1i8;
}

impl One for i16 {
    const ONE: Self = 1i16;
}

impl One for i32 {
    const ONE: Self = 1i32;
}

impl One for i64 {
    const ONE: Self = 1i64;
}

impl One for f32 {
    const ONE: Self = 1f32;
}

impl One for f64 {
    const ONE: Self = 1f64;
}

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

pub trait Num : Copy + Clone + Debug + Zero + One + NumOps + PartialOrd  {

}

impl Num for i8 {}
impl Num for i16 {}
impl Num for i32 {}
impl Num for i64 {}
impl Num for f32 {}
impl Num for f64 {}

pub trait Float : Num {
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
