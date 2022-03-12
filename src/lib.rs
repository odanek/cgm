#[macro_use]
mod macros;

mod angle;
mod crevice; // TODO Make this a feature, consider using encase
mod euler;
mod matrix;
mod num;
mod projection;
mod quaternion;
mod structure;
mod vector;

pub use angle::{Angle, Deg, Rad};
pub use euler::Euler;
pub use matrix::{Mat2, Mat3, Mat4, Matrix, SquareMatrix};
pub use num::{Float, Num, One, Signed, Zero};
pub use projection::{Ortho, Perspective};
pub use quaternion::Quat;
pub use structure::{ElementWise, InnerSpace, MetricSpace, VectorSpace};
pub use vector::{Vec1, Vec2, Vec3, Vec4, Vector};
