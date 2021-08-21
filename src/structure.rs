use std::ops::{Add, Div, Mul, Sub};

use crate::{Angle, Float, Num, One, Rad, Zero};

pub trait VectorSpace: Copy + Clone
where
    Self: Zero,
    Self: Add<Self, Output = Self>,
    Self: Sub<Self, Output = Self>,
    Self: Mul<<Self as VectorSpace>::Scalar, Output = Self>,
    Self: Div<<Self as VectorSpace>::Scalar, Output = Self>,
{
    type Scalar: Num;
}

pub trait MetricSpace: Sized {
    type Metric;

    fn distance2(self, other: Self) -> Self::Metric;

    fn distance(self, other: Self) -> Self::Metric
    where
        Self::Metric: Float,
    {
        self.distance2(other).sqrt()
    }
}

pub trait InnerSpace: VectorSpace
where
    Self: MetricSpace<Metric = <Self as VectorSpace>::Scalar>,
{
    fn dot(self, other: Self) -> Self::Scalar;

    #[inline]
    fn magnitude2(self) -> Self::Scalar {
        self.dot(self)
    }

    fn angle(self, other: Self) -> Rad<Self::Scalar>
    where
        Self::Scalar: Float,
    {
        Rad::acos(self.dot(other) / (self.magnitude() * other.magnitude()))
    }

    #[inline]
    fn project_on(self, other: Self) -> Self {
        other * (self.dot(other) / other.magnitude2())
    }

    #[inline]
    fn magnitude(self) -> Self::Scalar
    where
        Self::Scalar: Float,
    {
        self.magnitude2().sqrt()
    }

    #[inline]
    fn normalize(self) -> Self
    where
        Self::Scalar: Float,
    {
        self.normalize_to(<Self::Scalar as One>::ONE)
    }

    #[inline]
    fn normalize_to(self, magnitude: Self::Scalar) -> Self
    where
        Self::Scalar: Float,
    {
        self * (magnitude / self.magnitude())
    }
}

pub trait ElementWise<Rhs = Self> {
    fn add_element_wise(self, rhs: Rhs) -> Self;
    fn sub_element_wise(self, rhs: Rhs) -> Self;
    fn mul_element_wise(self, rhs: Rhs) -> Self;
    fn div_element_wise(self, rhs: Rhs) -> Self;
    fn rem_element_wise(self, rhs: Rhs) -> Self;

    fn add_assign_element_wise(&mut self, rhs: Rhs);
    fn sub_assign_element_wise(&mut self, rhs: Rhs);
    fn mul_assign_element_wise(&mut self, rhs: Rhs);
    fn div_assign_element_wise(&mut self, rhs: Rhs);
    fn rem_assign_element_wise(&mut self, rhs: Rhs);
}
