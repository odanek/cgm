use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};

use crate::{Float, Zero};

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
        if rem < Self::ZERO {
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
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Deg<S>(pub S);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Rad<S>(pub S);

impl<S: Float> From<Rad<S>> for Deg<S>
where
    Rad<S>: Angle,
    Deg<S>: Angle,
{
    #[inline]
    fn from(rad: Rad<S>) -> Deg<S> {
        Deg(rad.0 / S::DEG_RAD_RATIO)
    }
}

impl<S: Float> From<Deg<S>> for Rad<S>
where
    Deg<S>: Angle,
    Rad<S>: Angle,
{
    #[inline]
    fn from(deg: Deg<S>) -> Rad<S> {
        Rad(deg.0 * S::DEG_RAD_RATIO)
    }
}

impl<S: Float> Zero for Rad<S> {
    const ZERO: Self = Rad(S::ZERO);
}

impl<S: Float> Angle for Rad<S> {
    type Unitless = S;

    const FULL_TURN: Rad<S> = Rad(S::RAD_FULL_TURN);
    const HALF_TURN: Rad<S> = Rad(S::RAD_HALF_TURN);

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

impl_operator!(<S: Float> Add<Rad<S>> for Rad<S> {
    fn add(lhs, rhs) -> Rad<S> { Rad(lhs.0 + rhs.0) }
});
impl_operator!(<S: Float> Sub<Rad<S>> for Rad<S> {
    fn sub(lhs, rhs) -> Rad<S> { Rad(lhs.0 - rhs.0) }
});
impl_operator!(<S: Float> Mul<S> for Rad<S> {
    fn mul(lhs, rhs) -> Rad<S> { Rad(lhs.0 * rhs) }
});
impl_operator!(<S: Float> Div<Rad<S>> for Rad<S> {
    fn div(lhs, rhs) -> S { lhs.0 / rhs.0 }
});
impl_operator!(<S: Float> Div<S> for Rad<S> {
    fn div(lhs, rhs) -> Rad<S> { Rad(lhs.0 / rhs) }
});
impl_operator!(<S: Float> Rem<Rad<S>> for Rad<S> {
    fn rem(lhs, rhs) -> Rad<S> { Rad(lhs.0 % rhs.0) }
});
impl_assignment_operator!(<S: Float> AddAssign<Rad<S> > for Rad<S> {
    fn add_assign(&mut self, other) { self.0 += other.0; }
});
impl_assignment_operator!(<S: Float> SubAssign<Rad<S> > for Rad<S> {
    fn sub_assign(&mut self, other) { self.0 -= other.0; }
});
impl_assignment_operator!(<S: Float> RemAssign<Rad<S> > for Rad<S> {
    fn rem_assign(&mut self, other) { self.0 %= other.0; }
});
impl_assignment_operator!(<S: Float> MulAssign<S> for Rad<S> {
    fn mul_assign(&mut self, scalar) { self.0 *= scalar; }
});
impl_assignment_operator!(<S: Float> DivAssign<S> for Rad<S> {
    fn div_assign(&mut self, scalar) { self.0 /= scalar; }
});

impl<S: Float> Neg for Rad<S> {
    type Output = Rad<S>;

    #[inline]
    fn neg(self) -> Self::Output {
        Rad(-self.0)
    }
}

impl<'a, S: Float> Neg for &'a Rad<S> {
    type Output = Rad<S>;

    #[inline]
    fn neg(self) -> Self::Output {
        Rad(-self.0)
    }
}

impl<S: Float> Zero for Deg<S> {
    const ZERO: Self = Deg(S::ZERO);
}

impl<S: Float> Angle for Deg<S> {
    type Unitless = S;

    const FULL_TURN: Deg<S> = Deg(S::DEG_FULL_TURN);
    const HALF_TURN: Deg<S> = Deg(S::DEG_HALF_TURN);

    #[inline]
    fn sin(self) -> Self::Unitless {
        Rad::from(self).sin()
    }

    #[inline]
    fn cos(self) -> Self::Unitless {
        Rad::from(self).cos()
    }

    #[inline]
    fn tan(self) -> Self::Unitless {
        Rad::from(self).tan()
    }

    #[inline]
    fn asin(ratio: Self::Unitless) -> Self {
        Rad(ratio.asin()).into()
    }

    #[inline]
    fn acos(ratio: Self::Unitless) -> Self {
        Rad(ratio.acos()).into()
    }

    #[inline]
    fn atan(ratio: Self::Unitless) -> Self {
        Rad(ratio.atan()).into()
    }

    #[inline]
    fn atan2(a: Self::Unitless, b: Self::Unitless) -> Self {
        Rad(a.atan2(b)).into()
    }
}

impl_operator!(<S: Float> Add<Deg<S>> for Deg<S> {
    fn add(lhs, rhs) -> Deg<S> { Deg(lhs.0 + rhs.0) }
});
impl_operator!(<S: Float> Sub<Deg<S>> for Deg<S> {
    fn sub(lhs, rhs) -> Deg<S> { Deg(lhs.0 - rhs.0) }
});
impl_operator!(<S: Float> Mul<S> for Deg<S> {
    fn mul(lhs, rhs) -> Deg<S> { Deg(lhs.0 * rhs) }
});
impl_operator!(<S: Float> Div<Deg<S>> for Deg<S> {
    fn div(lhs, rhs) -> S { lhs.0 / rhs.0 }
});
impl_operator!(<S: Float> Div<S> for Deg<S> {
    fn div(lhs, rhs) -> Deg<S> { Deg(lhs.0 / rhs) }
});
impl_operator!(<S: Float> Rem<Deg<S>> for Deg<S> {
    fn rem(lhs, rhs) -> Deg<S> { Deg(lhs.0 % rhs.0) }
});
impl_assignment_operator!(<S: Float> AddAssign<Deg<S> > for Deg<S> {
    fn add_assign(&mut self, other) { self.0 += other.0; }
});
impl_assignment_operator!(<S: Float> SubAssign<Deg<S> > for Deg<S> {
    fn sub_assign(&mut self, other) { self.0 -= other.0; }
});
impl_assignment_operator!(<S: Float> RemAssign<Deg<S> > for Deg<S> {
    fn rem_assign(&mut self, other) { self.0 %= other.0; }
});
impl_assignment_operator!(<S: Float> MulAssign<S> for Deg<S> {
    fn mul_assign(&mut self, scalar) { self.0 *= scalar; }
});
impl_assignment_operator!(<S: Float> DivAssign<S> for Deg<S> {
    fn div_assign(&mut self, scalar) { self.0 /= scalar; }
});

impl<S: Float> Neg for Deg<S> {
    type Output = Deg<S>;

    #[inline]
    fn neg(self) -> Self::Output {
        Deg(-self.0)
    }
}

impl<'a, S: Float> Neg for &'a Deg<S> {
    type Output = Deg<S>;

    #[inline]
    fn neg(self) -> Self::Output {
        Deg(-self.0)
    }
}
