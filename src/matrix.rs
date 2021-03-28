use crate::{Angle, Float, Num, One, Rad, Vec2, Vec3, Vec4, Zero};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Mat2<S> {
    pub x: Vec2<S>,
    pub y: Vec2<S>,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Mat3<S> {
    pub x: Vec3<S>,
    pub y: Vec3<S>,
    pub z: Vec3<S>,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct Mat4<S> {
    pub x: Vec4<S>,
    pub y: Vec4<S>,
    pub z: Vec4<S>,
    pub w: Vec4<S>,
}

pub trait Matrix {
    type Row;
    type Column;
    type Transpose;

    fn row(&self, r: usize) -> Self::Row;
    fn column(&self, c: usize) -> Self::Column;
    fn transpose(&self) -> Self::Transpose;
}

impl<S: Zero + One> Mat2<S> {
    pub const IDENTITY: Mat2<S> = Mat2::from_cols(Vec2::X, Vec2::Y);
}

impl<S: Zero> Zero for Mat2<S> {
    const ZERO: Mat2<S> = Mat2::from_cols(Vec2::ZERO, Vec2::ZERO);
}

impl<S> Mat2<S> {
    #[inline]
    pub const fn new(c0r0: S, c0r1: S, c1r0: S, c1r1: S) -> Mat2<S> {
        Self::from_cols(Vec2::new(c0r0, c0r1), Vec2::new(c1r0, c1r1))
    }

    #[inline]
    pub const fn from_cols(x: Vec2<S>, y: Vec2<S>) -> Mat2<S> {
        Mat2 { x, y }
    }
}

impl<S: Float> Mat2<S> {
    #[inline]
    pub fn from_translation(t: S) -> Mat2<S> {
        Mat2::new(S::ONE, S::ZERO, t, S::ONE)
    }

    #[inline]
    pub fn from_scale(value: S) -> Mat2<S> {
        Mat2::new(value, S::ZERO, S::ZERO, value)
    }

    #[inline]
    pub fn from_nonuniform_scale(x: S, y: S) -> Mat2<S> {
        Mat2::new(x, S::ZERO, S::ZERO, y)
    }

    #[inline]
    pub fn from_rotation<A: Into<Rad<S>>>(theta: A) -> Mat2<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();
        Mat2::new(c, s, -s, c)
    }
}

impl<S: Num> Matrix for Mat2<S> {
    type Row = Vec2<S>;
    type Column = Vec2<S>;
    type Transpose = Mat2<S>;

    #[inline]
    fn row(&self, r: usize) -> Vec2<S> {
        match r {
            0 => Vec2::new(self.x.x, self.y.x),
            1 => Vec2::new(self.x.y, self.y.y),
            _ => panic!("Invalid row index"),
        }
    }

    #[inline]
    fn column(&self, r: usize) -> Vec2<S> {
        match r {
            0 => self.x,
            1 => self.y,
            _ => panic!("Invalid column index"),
        }
    }

    fn transpose(&self) -> Mat2<S> {
        Mat2::new(self.x.x, self.y.x, self.x.y, self.y.y)
    }
}

impl<S: Zero> Zero for Mat3<S> {
    const ZERO: Mat3<S> = Mat3::from_cols(Vec3::ZERO, Vec3::ZERO, Vec3::ZERO);
}

impl<S: Zero + One> Mat3<S> {
    pub const IDENTITY: Mat3<S> = Mat3::from_cols(Vec3::X, Vec3::Y, Vec3::Z);
}

impl<S> Mat3<S> {
    #[inline]
    #[rustfmt::skip]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        c0r0:S, c0r1:S, c0r2:S,
        c1r0:S, c1r1:S, c1r2:S,
        c2r0:S, c2r1:S, c2r2:S,
    ) -> Mat3<S> {
        Mat3::from_cols(
            Vec3::new(c0r0, c0r1, c0r2),
            Vec3::new(c1r0, c1r1, c1r2),
            Vec3::new(c2r0, c2r1, c2r2),
        )
    }

    #[inline]
    pub const fn from_cols(x: Vec3<S>, y: Vec3<S>, z: Vec3<S>) -> Mat3<S> {
        Mat3 { x, y, z }
    }
}

impl<S: Float> Mat3<S> {
    #[inline]
    #[rustfmt::skip]
    pub fn from_translation(v: Vec2<S>) -> Mat3<S> {
        Mat3::new(
            S::ONE, S::ZERO, S::ZERO,
            S::ZERO, S::ONE, S::ZERO,
            v.x, v.y, S::ONE,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_scale(value: S) -> Mat3<S> {
        Mat3::new(
            value, S::ZERO, S::ZERO,
            S::ZERO, value, S::ZERO,
            S::ZERO, S::ZERO, value,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_nonuniform_scale(x: S, y: S, z: S) -> Mat3<S> {
        Mat3::new(
            x, S::ZERO, S::ZERO,
            S::ZERO, y, S::ZERO,
            S::ZERO, S::ZERO, z,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation<A: Into<Rad<S>>>(axis: Vec3<S>, theta: A) -> Mat3<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();
        let mc = S::ONE - c;

        Mat3::new(
            mc * axis.x * axis.x + c,
            mc * axis.x * axis.y + s * axis.z,
            mc * axis.x * axis.z - s * axis.y,

            mc * axis.x * axis.y - s * axis.z,
            mc * axis.y * axis.y + c,
            mc * axis.y * axis.z + s * axis.x,

            mc * axis.x * axis.z + s * axis.y,
            mc * axis.y * axis.z - s * axis.x,
            mc * axis.z * axis.z + c,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation_x<A: Into<Rad<S>>>(theta: A) -> Mat3<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();

        Mat3::new(
            S::ONE, S::ZERO, S::ZERO,
            S::ZERO, c, s,
            S::ZERO, -s, c,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation_y<A: Into<Rad<S>>>(theta: A) -> Mat3<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();

        Mat3::new(
            c, S::ZERO, s,
            S::ZERO, S::ONE, S::ZERO,
            -s, S::ZERO, c,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation_z<A: Into<Rad<S>>>(theta: A) -> Mat3<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();

        Mat3::new(
            c, s, S::ZERO,
            -s, c, S::ZERO,
            S::ZERO, S::ZERO, S::ONE,
        )
    }
}

impl<S: Num> Matrix for Mat3<S> {
    type Row = Vec3<S>;
    type Column = Vec3<S>;
    type Transpose = Mat3<S>;

    #[inline]
    fn row(&self, r: usize) -> Vec3<S> {
        match r {
            0 => Vec3::new(self.x.x, self.y.x, self.z.x),
            1 => Vec3::new(self.x.y, self.y.y, self.z.y),
            2 => Vec3::new(self.x.z, self.y.z, self.z.z),
            _ => panic!("Invalid row index"),
        }
    }

    #[inline]
    fn column(&self, r: usize) -> Vec3<S> {
        match r {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Invalid column index"),
        }
    }

    #[rustfmt::skip]
    fn transpose(&self) -> Mat3<S> {
        Mat3::new(
            self.x.x, self.y.x, self.z.x,
            self.x.y, self.y.y, self.z.y,
            self.x.z, self.y.z, self.z.z,
        )
    }
}

impl<S: Zero> Zero for Mat4<S> {
    const ZERO: Mat4<S> = Mat4::from_cols(Vec4::ZERO, Vec4::ZERO, Vec4::ZERO, Vec4::ZERO);
}

impl<S: Zero + One> Mat4<S> {
    pub const IDENTITY: Mat4<S> = Mat4::from_cols(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W);
}

impl<S> Mat4<S> {
    #[inline]
    #[rustfmt::skip]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        c0r0: S, c0r1: S, c0r2: S, c0r3: S,
        c1r0: S, c1r1: S, c1r2: S, c1r3: S,
        c2r0: S, c2r1: S, c2r2: S, c2r3: S,
        c3r0: S, c3r1: S, c3r2: S, c3r3: S,
    ) -> Mat4<S>  {
        Mat4::from_cols(
            Vec4::new(c0r0, c0r1, c0r2, c0r3),
            Vec4::new(c1r0, c1r1, c1r2, c1r3),
            Vec4::new(c2r0, c2r1, c2r2, c2r3),
            Vec4::new(c3r0, c3r1, c3r2, c3r3),
        )
    }

    #[inline]
    pub const fn from_cols(x: Vec4<S>, y: Vec4<S>, z: Vec4<S>, w: Vec4<S>) -> Mat4<S> {
        Mat4 { x, y, z, w }
    }
}

impl<S: Float> Mat4<S> {
    #[inline]
    #[rustfmt::skip]
    pub fn from_translation(v: Vec3<S>) -> Mat4<S> {
        Mat4::new(
            S::ONE, S::ZERO, S::ZERO, S::ZERO,
            S::ZERO, S::ONE, S::ZERO, S::ZERO,
            S::ZERO, S::ZERO, S::ONE, S::ZERO,
            v.x, v.y, v.z, S::ONE,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_scale(value: S) -> Mat4<S> {
        Mat4::new(
            value, S::ZERO, S::ZERO, S::ZERO,
            S::ZERO, value, S::ZERO, S::ZERO,
            S::ZERO, S::ZERO, value, S::ZERO,
            S::ZERO, S::ZERO, S::ZERO, S::ONE,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_nonuniform_scale(x: S, y: S, z: S) -> Mat4<S> {
        Mat4::new(
            x, S::ZERO, S::ZERO, S::ZERO,
            S::ZERO, y, S::ZERO, S::ZERO,
            S::ZERO, S::ZERO, z, S::ZERO,
            S::ZERO, S::ZERO, S::ZERO, S::ONE,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation<A: Into<Rad<S>>>(axis: Vec3<S>, theta: A) -> Mat4<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();
        let mc = S::ONE - c;

        Mat4::new(
            mc * axis.x * axis.x + c,
            mc * axis.x * axis.y + s * axis.z,
            mc * axis.x * axis.z - s * axis.y,
            S::ZERO,

            mc * axis.x * axis.y - s * axis.z,
            mc * axis.y * axis.y + c,
            mc * axis.y * axis.z + s * axis.x,
            S::ZERO,

            mc * axis.x * axis.z + s * axis.y,
            mc * axis.y * axis.z - s * axis.x,
            mc * axis.z * axis.z + c,
            S::ZERO,

            S::ZERO, S::ZERO, S::ZERO, S::ONE,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation_x<A: Into<Rad<S>>>(theta: A) -> Mat4<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();

        Mat4::new(
            S::ONE, S::ZERO, S::ZERO, S::ZERO,
            S::ZERO, c, s, S::ZERO,
            S::ZERO, -s, c, S::ZERO,
            S::ZERO, S::ZERO, S::ZERO, S::ONE,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation_y<A: Into<Rad<S>>>(theta: A) -> Mat4<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();

        Mat4::new(
            c, S::ZERO, s,S::ZERO,
            S::ZERO, S::ONE, S::ZERO, S::ZERO,
            -s, S::ZERO, c, S::ZERO,
            S::ZERO, S::ZERO, S::ZERO, S::ONE,
        )
    }

    #[inline]
    #[rustfmt::skip]
    pub fn from_rotation_z<A: Into<Rad<S>>>(theta: A) -> Mat4<S> {
        let angle: Rad<S> = theta.into();
        let s = angle.sin();
        let c = angle.cos();

        Mat4::new(
            c, s, S::ZERO, S::ZERO,
            -s, c, S::ZERO, S::ZERO,
            S::ZERO, S::ZERO, S::ONE, S::ZERO,
            S::ZERO, S::ZERO, S::ZERO, S::ONE,
        )
    }
}

impl<S: Num> Matrix for Mat4<S> {
    type Row = Vec4<S>;
    type Column = Vec4<S>;
    type Transpose = Mat4<S>;

    #[inline]
    fn row(&self, r: usize) -> Vec4<S> {
        match r {
            0 => Vec4::new(self.x.x, self.y.x, self.z.x, self.w.x),
            1 => Vec4::new(self.x.y, self.y.y, self.z.y, self.w.y),
            2 => Vec4::new(self.x.z, self.y.z, self.z.z, self.w.z),
            3 => Vec4::new(self.x.w, self.y.w, self.z.w, self.w.w),
            _ => panic!("Invalid row index"),
        }
    }

    #[inline]
    fn column(&self, r: usize) -> Vec4<S> {
        match r {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => panic!("Invalid column index"),
        }
    }

    #[rustfmt::skip]
    fn transpose(&self) -> Mat4<S> {
        Mat4::new(
            self.x.x, self.y.x, self.z.x, self.w.x,
            self.x.y, self.y.y, self.z.y, self.w.y,
            self.x.z, self.y.z, self.z.z, self.w.z,
            self.x.w, self.y.w, self.z.w, self.w.w,
        )
    }
}

// impl Mat4 {
//     pub fn perspective(fov: Rad<f32>, aspect: f32, near_clip: f32, far_clip: f32) -> Mat4 {
//         let half_fov = fov.0 / 2.0;
//         let f = half_fov.cos() / half_fov.sin();
//         let d = far_clip - near_clip;

//         Mat4 {
//             #[rustfmt::skip]
//             data: [
//                 f / aspect, 0.0, 0.0, 0.0,
//                 0.0, -f, 0.0, 0.0,
//                 0.0, 0.0, -far_clip / d, -1.0,
//                 0.0, 0.0, -(far_clip * near_clip) / d, 0.0
//             ]
//         }
//     }

//     pub fn ortho(left: f32, right: f32, top: f32, bottom: f32, near: f32, far: f32) -> Mat4 {
//         let x = right - left;
//         let y= top - bottom;
//         let z= far - near;

//         Mat4 {
//             #[rustfmt::skip]
//             data: [
//                 2.0 / x, 0.0, 0.0, 0.0,
//                 0.0, 2.0 / y, 0.0, 0.0,
//                 0.0, 0.0, -2.0 / z, 0.0,
//                 (left + right) / -x, (bottom + top) / -y, (near + far) / -z, 1.0
//             ]
//         }
//     }

//     pub fn look_at(eye: Vec3<f32>, center: Vec3<f32>, up: Vec3<f32>) -> Mat4 {
//         let f = (center - eye).normalize();
//         let s = f.cross(up.normalize());
//         let u = s.normalize().cross(f);
//         let m = Mat4 {
//             #[rustfmt::skip]
//             data: [
//                 s.x, u.x, -f.x, 0.0,
//                 s.y, u.y, -f.y, 0.0,
//                 s.z, u.z, -f.z, 0.0,
//                 0.0, 0.0, 0.0, 1.0
//             ]
//         };
//         m * Mat4::vec_translate(-eye)
//     }
// }

// impl ops::Add<Mat4> for Mat4 {
//     type Output = Mat4;

//     fn add(self, rhs: Mat4) -> Self::Output {
//         Mat4 {
//             data: [
//                 self.data[0] + rhs.data[0],
//                 self.data[1] + rhs.data[1],
//                 self.data[2] + rhs.data[2],
//                 self.data[3] + rhs.data[3],
//                 self.data[4] + rhs.data[4],
//                 self.data[5] + rhs.data[5],
//                 self.data[6] + rhs.data[6],
//                 self.data[7] + rhs.data[7],
//                 self.data[8] + rhs.data[8],
//                 self.data[9] + rhs.data[9],
//                 self.data[10] + rhs.data[10],
//                 self.data[11] + rhs.data[11],
//                 self.data[12] + rhs.data[12],
//                 self.data[13] + rhs.data[13],
//                 self.data[14] + rhs.data[14],
//                 self.data[15] + rhs.data[15],
//             ]
//         }
//     }
// }

// impl ops::Mul<Mat4> for Mat4 {
//     type Output = Mat4;

//     fn mul(self, rhs: Mat4) -> Self::Output {
//         &self * &rhs
//     }
// }

// impl ops::Mul<&Mat4> for Mat4 {
//     type Output = Mat4;

//     fn mul(self, rhs: &Mat4) -> Self::Output {
//         &self * rhs
//     }
// }

// impl ops::Mul<Mat4> for &Mat4 {
//     type Output = Mat4;

//     fn mul(self, rhs: Mat4) -> Self::Output {
//         self * &rhs
//     }
// }

// impl ops::Mul<&Mat4> for &Mat4 {
//     type Output = Mat4;

//     fn mul(self, rhs: &Mat4) -> Self::Output {
//         let mut data: [f32; 16] = [0f32; 16]; // TODO Not necessary
//         let mut index = 0usize;

//         for column in 0..4 {
//             for row in 0..4 {
//                 let mut left = row;
//                 let mut right = column * 4;
//                 let mut sum = 0f32;

//                 for _ in 0..4 {
//                     sum += self.data[left] * rhs.data[right];
//                     left += 4;
//                     right += 1;
//                 }
//                 data[index] = sum;
//                 index += 1;
//             }
//         }

//         Mat4 {
//             data
//         }
//     }
// }

// impl ops::Mul<Vec4<f32>> for Mat4 {
//     type Output = Vec4<f32>;

//     fn mul(self, rhs: Vec4<f32>) -> Self::Output {
//         &self * &rhs
//     }
// }

// impl ops::Mul<&Vec4<f32>> for Mat4 {
//     type Output = Vec4<f32>;

//     fn mul(self, rhs: &Vec4<f32>) -> Self::Output {
//         &self * rhs
//     }
// }

// impl ops::Mul<Vec4<f32>> for &Mat4 {
//     type Output = Vec4<f32>;

//     fn mul(self, rhs: Vec4<f32>) -> Self::Output {
//         self * &rhs
//     }
// }

// impl ops::Mul<&Vec4<f32>> for &Mat4 {
//     type Output = Vec4<f32>;

//     fn mul(self, rhs: &Vec4<f32>) -> Self::Output {
//         let m = &self.data;
//         let x = rhs.x;
//         let y = rhs.y;
//         let z = rhs.z;
//         let w = rhs.w;

//         #[rustfmt::skip]
//         Vec4::new(
//             m[0] * x + m[4] * y + m[8] * z + m[12] * w,
//             m[1] * x + m[5] * y + m[9] * z + m[13] * w,
//             m[2] * x + m[6] * y + m[10] * z + m[14] * w,
//             m[3] * x + m[7] * y + m[11] * z + m[15] * w,
//         )
//     }
// }
