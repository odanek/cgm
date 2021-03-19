use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use crate::{Float, Num, Signed, Zero};

pub trait Angle
where
    Self: Copy + Clone + PartialEq + PartialOrd,
    Self: Zero,
    Self: Neg<Output = Self>,
    Self: Add<Self, Output = Self>,
    Self: Sub<Self, Output = Self>,
    Self: Rem<Self, Output = Self>,
    Self: Mul<<Self as Angle>::Unitless, Output = Self>,
    Self: Div<Self, Output = <Self as Angle>::Unitless>,
    Self: Div<<Self as Angle>::Unitless, Output = Self>,
{
    type Unitless: Float;

    const TURN: Self;
    const TURN_2: Self;
    const TURN_3: Self;
    const TURN_4: Self;
    const TURN_6: Self;

    #[inline]
    fn normalize(self) -> Self {
        let rem = self % Self::TURN;
        if rem < <Self as Zero>::ZERO {
            rem + Self::TURN
        } else {
            rem
        }
    }

    #[inline]
    fn normalize_signed(self) -> Self {
        let rem = self.normalize();
        if Self::TURN_2 < rem {
            rem - Self::TURN
        } else {
            rem
        }
    }

    #[inline]
    fn opposite(self) -> Self {
        (self + Self::TURN_2).normalize()
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
        Deg(rad.0 * (Deg::<S>::TURN_2).0 / (Rad::<S>::TURN_2).0)
    }
}

impl<S: Float> From<Deg<S>> for Rad<S>
where
    Deg<S>: Angle,
    Rad<S>: Angle,
{
    #[inline]
    fn from(deg: Deg<S>) -> Rad<S> {
        Rad(deg.0 * (Rad::<S>::TURN_2).0 / (Deg::<S>::TURN_2).0)
    }
}

impl<S: Num> Zero for Rad<S> {
    const ZERO: Self = Rad(<S as Zero>::ZERO);
}

impl<S: Signed> Neg for Rad<S> {
    type Output = Rad<S>;

    #[inline]
    fn neg(self) -> Rad<S> { Rad(-self.0) }
}

impl<'a, S: Signed> Neg for &'a Rad<S> {
    type Output = Rad<S>;

    #[inline]
    fn neg(self) -> Rad<S> { Rad(-self.0) }
}

macro_rules! impl_radians {
    ($Angle:ident<$S: ty>, $turn_2:expr) => {
        impl Angle for $Angle<$S> {
            type Unitless = $S;
        
            const TURN: $Angle<$S> = $Angle(2.0 * $turn_2);
            const TURN_2: $Angle<$S> = $Angle($turn_2);
            const TURN_3: $Angle<$S> = $Angle(2.0 * $turn_2 / 3.0);
            const TURN_4: $Angle<$S> = $Angle($turn_2 / 2.0);
            const TURN_6: $Angle<$S> = $Angle($turn_2 / 3.0);
        
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
        
        // impl_operator!(<S: Float>, Add<Rad<S>>, Rad<S>, {
        //     fn add(lhs, rhs) -> Rad<S> { Rad(lhs.0 + rhs.0) }
        // });
        // impl_operator!(<S: Float>, Sub<Rad<S>>, Rad<S>, {
        //     fn sub(lhs, rhs) -> Rad<S> { Rad(lhs.0 - rhs.0) }
        // });
        // impl_operator!(<S: Float>, Mul<S>, Rad<S>, {
        //     fn mul(lhs, rhs) -> Rad<S> { Rad(lhs.0 * rhs) }
        // });
        // impl_operator!(<S: Float>, Div<Rad<S>>, Rad<S>, {
        //     fn div(lhs, rhs) -> S { lhs.0 / rhs.0 }
        // });
        // impl_operator!(<S: Float>, Div<S>, Rad<S>, {
        //     fn div(lhs, rhs) -> Rad<S> { Rad(lhs.0 / rhs) }
        // });
        // impl_operator!(<S: Float>, Rem<Rad<S>>, Rad<S>, {
        //     fn rem(lhs, rhs) -> Rad<S> { Rad(lhs.0 % rhs.0) }
        // });        
    }
}

impl_radians!(Rad<f32>, std::f32::consts::PI);
impl_radians!(Rad<f64>, std::f64::consts::PI);