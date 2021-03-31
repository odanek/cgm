#[macro_use]
mod macros;

mod angle;
mod matrix;
mod num;
mod projection;
mod space;
mod vector;

pub use angle::{Angle, Deg, Rad};
pub use matrix::{Mat2, Mat3, Mat4};
pub use num::{Float, Num, One, SignedNum, Zero};
pub use projection::{Ortho, Perspective};
pub use space::{InnerSpace, MetricSpace, VectorSpace};
pub use vector::{Vec1, Vec2, Vec3, Vec4};
