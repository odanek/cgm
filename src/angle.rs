use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use crate::{Float, Num, Zero};

pub trait Angle
where
    Self: Copy + Clone + PartialEq + PartialOrd + Zero,
    Self: Neg<Output = Self>,
    Self: Add<Self, Output = Self>,
    Self: Sub<Self, Output = Self>,
    Self: Rem<Self, Output = Self>,
    Self: Mul<<Self as Angle>::Unitless, Output = Self>,
    Self: Div<Self, Output = <Self as Angle>::Unitless>,
    Self: Div<<Self as Angle>::Unitless, Output = Self>,
{
    type Unitless: Float;

    const FULL_TURN: Self;
    const HALF_TURN: Self;

    #[inline]
    fn normalize(self) -> Self {
        let rem = self % Self::FULL_TURN;
        if rem < <Self as Zero>::ZERO {
            rem + Self::FULL_TURN
        } else {
            rem
        }
    }

    #[inline]
    fn normalize_signed(self) -> Self {
        let rem = self.normalize();
        if Self::HALF_TURN < rem {
            rem - Self::FULL_TURN
        } else {
            rem
        }
    }

    #[inline]
    fn opposite(self) -> Self {
        (self + Self::HALF_TURN).normalize()
    }

    fn sin(self) -> Self::Unitless;

    fn cos(self) -> Self::Unitless;

    fn tan(self) -> Self::Unitless;

    fn asin(ratio: Self::Unitless) -> Self;

    fn acos(ratio: Self::Unitless) -> Self;

    fn atan(ratio: Self::Unitless) -> Self;

    fn atan2(a: Self::Unitless, b: Self::Unitless) -> Self;
}

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Deg<S>(pub S);

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Rad<S>(pub S);

impl<S: Float> From<Rad<S>> for Deg<S>
where
    Rad<S>: Angle,
    Deg<S>: Angle,
{
    #[inline]
    fn from(rad: Rad<S>) -> Deg<S> {
        Deg(rad.0 * <S as Float>::RAD_HALF_TURN / <S as Float>::DEG_HALF_TURN)
    }
}

impl<S: Float> From<Deg<S>> for Rad<S>
where
    Deg<S>: Angle,
    Rad<S>: Angle,
{
    #[inline]
    fn from(deg: Deg<S>) -> Rad<S> {
        Rad(deg.0 * <S as Float>::DEG_HALF_TURN / <S as Float>::RAD_HALF_TURN)
    }
}

impl<S: Num> Zero for Rad<S> {
    const ZERO: Self = Rad(<S as Zero>::ZERO);
}

impl<S: Float> Angle for Rad<S> {
    type Unitless = S;

    const FULL_TURN: Rad<S> = Rad(<S as Float>::RAD_FULL_TURN);
    const HALF_TURN: Rad<S> = Rad(<S as Float>::RAD_HALF_TURN);

    #[inline]
    fn sin(self) -> Self::Unitless {
        self.0.sin()
    }

    #[inline]
    fn cos(self) -> Self::Unitless {
        self.0.cos()
    }

    #[inline]
    fn tan(self) -> Self::Unitless {
        self.0.tan()
    }

    #[inline]
    fn asin(ratio: Self::Unitless) -> Self {
        Rad(ratio.asin())
    }

    #[inline]
    fn acos(ratio: Self::Unitless) -> Self {
        Rad(ratio.acos())
    }

    #[inline]
    fn atan(ratio: Self::Unitless) -> Self {
        Rad(ratio.atan())
    }

    #[inline]
    fn atan2(a: Self::Unitless, b: Self::Unitless) -> Self {
        Rad(a.atan2(b))
    }
}

impl_operator!(<S: Float>, Add<Rad<S>>, Rad<S>, {
    fn add(lhs, rhs) -> Rad<S> { Rad(lhs.0 + rhs.0) }
});
impl_operator!(<S: Float>, Sub<Rad<S>>, Rad<S>, {
    fn sub(lhs, rhs) -> Rad<S> { Rad(lhs.0 - rhs.0) }
});
impl_operator!(<S: Float>, Mul<S>, Rad<S>, {
    fn mul(lhs, rhs) -> Rad<S> { Rad(lhs.0 * rhs) }
});
impl_operator!(<S: Float>, Div<Rad<S>>, Rad<S>, {
    fn div(lhs, rhs) -> S { lhs.0 / rhs.0 }
});
impl_operator!(<S: Float>, Div<S>, Rad<S>, {
    fn div(lhs, rhs) -> Rad<S> { Rad(lhs.0 / rhs) }
});
impl_operator!(<S: Float>, Rem<Rad<S>>, Rad<S>, {
    fn rem(lhs, rhs) -> Rad<S> { Rad(lhs.0 % rhs.0) }
});

impl<S: Float> Neg for Rad<S>
{
    type Output = Rad<S>;

    #[inline]
    fn neg(self) -> Self::Output {
        Rad(-self.0)
    }
}

impl<'a, S: Float> Neg for &'a Rad<S>
{
    type Output = Rad<S>;

    #[inline]
    fn neg(self) -> Self::Output {
        Rad(-self.0)
    }
}

impl<S: Num> Zero for Deg<S> {
    const ZERO: Self = Deg(<S as Zero>::ZERO);
}

impl<S: Float> Angle for Deg<S> {
    type Unitless = S;

    const FULL_TURN: Deg<S> = Deg(<S as Float>::DEG_FULL_TURN);
    const HALF_TURN: Deg<S> = Deg(<S as Float>::DEG_HALF_TURN);

    #[inline]
    fn sin(self) -> Self::Unitless {
        self.0.sin()
    }

    #[inline]
    fn cos(self) -> Self::Unitless {
        self.0.cos()
    }

    #[inline]
    fn tan(self) -> Self::Unitless {
        self.0.tan()
    }

    #[inline]
    fn asin(ratio: Self::Unitless) -> Self {
        Deg(ratio.asin())
    }

    #[inline]
    fn acos(ratio: Self::Unitless) -> Self {
        Deg(ratio.acos())
    }

    #[inline]
    fn atan(ratio: Self::Unitless) -> Self {
        Deg(ratio.atan())
    }

    #[inline]
    fn atan2(a: Self::Unitless, b: Self::Unitless) -> Self {
        Deg(a.atan2(b))
    }
}

impl_operator!(<S: Float>, Add<Deg<S>>, Deg<S>, {
    fn add(lhs, rhs) -> Deg<S> { Deg(lhs.0 + rhs.0) }
});
impl_operator!(<S: Float>, Sub<Deg<S>>, Deg<S>, {
    fn sub(lhs, rhs) -> Deg<S> { Deg(lhs.0 - rhs.0) }
});
impl_operator!(<S: Float>, Mul<S>, Deg<S>, {
    fn mul(lhs, rhs) -> Deg<S> { Deg(lhs.0 * rhs) }
});
impl_operator!(<S: Float>, Div<Deg<S>>, Deg<S>, {
    fn div(lhs, rhs) -> S { lhs.0 / rhs.0 }
});
impl_operator!(<S: Float>, Div<S>, Deg<S>, {
    fn div(lhs, rhs) -> Deg<S> { Deg(lhs.0 / rhs) }
});
impl_operator!(<S: Float>, Rem<Deg<S>>, Deg<S>, {
    fn rem(lhs, rhs) -> Deg<S> { Deg(lhs.0 % rhs.0) }
});

impl<S: Float> Neg for Deg<S>
{
    type Output = Deg<S>;

    #[inline]
    fn neg(self) -> Self::Output {
        Deg(-self.0)
    }
}

impl<'a, S: Float> Neg for &'a Deg<S>
{
    type Output = Deg<S>;

    #[inline]
    fn neg(self) -> Self::Output {
        Deg(-self.0)
    }
}