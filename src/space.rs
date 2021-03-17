use crate::{Float, One, Rad};

pub trait VectorSpace: Copy + Clone {
    type Scalar;
}

pub trait MetricSpace: Sized {
    type Metric;

    fn distance2(self, other: Self) -> Self::Metric;

    fn distance(self, other: Self) -> Self::Metric
    where
        Self::Metric: Float,
    {
        Self::distance2(self, other).sqrt()
    }
}

pub trait InnerSpace: VectorSpace
where
    Self: MetricSpace<Metric = <Self as VectorSpace>::Scalar>,
{
    fn dot(self, other: Self) -> Self::Scalar;

    #[inline]
    fn magnitude2(self) -> Self::Scalar {
        Self::dot(self, self)
    }

    fn angle(self, other: Self) -> Rad<Self::Scalar>
    where
        Self::Scalar: Float,
    {
        Rad((Self::dot(self, other) / (self.magnitude() * other.magnitude())).acos())
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
