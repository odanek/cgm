#[macro_use]
mod macros;

mod angle;
mod matrix;
mod num;
mod space;
mod vector;

pub use angle::{Angle, Deg, Rad};
pub use matrix::{Mat2, Mat3, Mat4};
pub use num::{Zero, One, NumOps, Num, SignedNum, Float};
pub use space::{VectorSpace, MetricSpace, InnerSpace};
pub use vector::{Vec1, Vec2, Vec3, Vec4};