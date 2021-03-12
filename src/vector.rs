use std::ops::{Add, Sub};

use crate::{Rad, num::{Float, Num, One, Zero}};

use super::Mat4;

#[repr(C)]
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct Vec1<S> {
    pub x: S,
}

#[repr(C)]
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct Vec2<S> {
    pub x: S,
    pub y: S,
}

#[repr(C)]
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct Vec3<S> {
    pub x: S,
    pub y: S,
    pub z: S,
}

#[repr(C)]
#[derive(Eq, PartialEq, Clone, Copy, Debug, Default)]
pub struct Vec4<S> {
    pub x: S,
    pub y: S,
    pub z: S,
    pub w: S,
}

macro_rules! impl_operator {
    (<$S:ident: $Constraint:ident>, $Op:ident<$Rhs:ty>, $Lhs:ty, {
        fn $op:ident($lhs: ident, $rhs: ident) -> $Out:ty { $body:expr }
    }) => {
        impl<$S: $Constraint> $Op<$Rhs> for $Lhs {
            type Output = $Out;
        
            fn $op(self, other: $Rhs) -> Self::Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }
        
        impl<'a, $S: $Constraint> $Op<$Rhs> for &'a $Lhs {
            type Output = $Out;
        
            fn $op(self, other: $Rhs) -> Self::Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a, $S: $Constraint> $Op<&'a $Rhs> for $Lhs {
            type Output = $Out;
        
            fn $op(self, other: &'a $Rhs) -> Self::Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }
        
        impl<'a, 'b, $S: $Constraint> $Op<&'a $Rhs> for &'b $Lhs {
            type Output = $Out;
        
            fn $op(self, other: &'a $Rhs) -> Self::Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }
    }
}

macro_rules! impl_vector {
    ($VecN:ident { $($field:ident),+ }, $n:expr) => {
        impl<S> $VecN<S> {
            #[inline]
            pub const fn new($($field: S),+) -> $VecN<S> {
                $VecN { $($field: $field),+ }
            }
        }

        impl<S: Zero> Zero for $VecN<S> {
            const ZERO: $VecN<S> = $VecN { $($field: S::ZERO),+ };
        }

        impl_operator!(<S: Num>, Add<$VecN<S>>, $VecN<S>, { 
            fn add(lhs, rhs) -> $VecN<S> { $VecN { $($field: lhs.$field + rhs.$field),+ } }
        });
        impl_operator!(<S: Num>, Sub<$VecN<S>>, $VecN<S>, { 
            fn sub(lhs, rhs) -> $VecN<S> { $VecN { $($field: lhs.$field - rhs.$field),+ } }
        });
    }
}

impl_vector!(Vec1 { x }, 1);
impl_vector!(Vec2 { x, y }, 2);
impl_vector!(Vec3 { x, y, z }, 3);
impl_vector!(Vec4 { x, y, z, w }, 4);

// pub const ZERO2F: Vec2<f32> = Vec2 { x: 0.0, y: 0.0 };
// pub const X2F: Vec2<f32> = Vec2 { x: 1.0, y: 0.0 };
// pub const Y2F: Vec2<f32> = Vec2 { x: 0.0, y: 1.0 };

// impl<S> Zero for Vec2<S> where S: Zero {
//     const ZERO: Vec2<S> = Vec2 { x: S::ZERO, y: S::ZERO };
// }

// impl<S> Vec2<S> where S: One + Zero {
//     const X: Vec2<S> = Vec2 { x: S::ONE, y: S::ZERO };
//     const Y: Vec2<S> = Vec2 { x: S::ZERO, y: S::ONE };
// }

// impl<S> Vec2<S> {    
//     pub const fn new(x: S, y: S) -> Vec2<S> {
//         Vec2 { x, y }
//     }
// }

// impl<S> Vec2<S> where S: Num {
//     pub fn dot(&self, rhs: &Vec2<S>) -> S {
//         self.x * rhs.x + self.y * rhs.y
//     }
// }

// impl<S> Vec2<S> where S: Float {
//     pub fn length(&self) -> S {
//         (self.x * self.x + self.y * self.y).sqrt()
//     }

//     pub fn unit(&self) -> Vec2<S> {
//         self / self.length()
//     }

//     // pub fn angle(&self) -> Rad {
//     // self.y.atan2(self.x)
//     // }
// }


// impl<S> ops::Mul<S> for Vec2<S> {
//     type Output = Vec2<S>;

//     fn mul(self, rhs: S) -> Self::Output {
//         &self * rhs
//     }
// }

// impl<S> ops::Mul<S> for &Vec2<S> {
//     type Output = Vec2<S>;

//     fn mul(self, rhs: S) -> Self::Output {
//         Vec2 {
//             x: self.x * rhs,
//             y: self.y * rhs,
//         }
//     }
// }

// impl<S> ops::Mul<Vec2<S>> for S where S: Num {
//     type Output = Vec2<S>;

//     fn mul(self, rhs: Vec2<S>) -> Self::Output {
//         self * &rhs
//     }
// }

// impl<S> ops::Mul<&Vec2<S>> for S {
//     type Output = Vec2<S>;

//     fn mul(self, rhs: &Vec2<S>) -> Self::Output {
//         Vec2 {
//             x: self * rhs.x,
//             y: self * rhs.y,
//         }
//     }
// }

// impl<S> ops::Div<S> for Vec2<S> {
//     type Output = Vec2<S>;

//     fn div(self, rhs: S) -> Self::Output {
//         &self / rhs
//     }
// }

// impl<S> ops::Div<S> for &Vec2<S> {
//     type Output = Vec2<S>;

//     fn div(self, rhs: S) -> Self::Output {
//         Vec2 {
//             x: self.x / rhs,
//             y: self.y / rhs,
//         }
//     }
// }

// impl<S> ops::Neg for Vec2<S> {
//     type Output = Vec2<S>;

//     fn neg(self) -> Self::Output {
//         -(&self)
//     }
// }

// impl<S> ops::Neg for &Vec2<S> {
//     type Output = Vec2<S>;

//     fn neg(self) -> Self::Output {
//         Vec2 {
//             x: -self.x,
//             y: -self.y,
//         }
//     }
// }

// #[repr(C)]
// #[derive(Clone, Copy, Debug, Default)]
// pub struct Vec3 {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
// }

// pub const ZERO3: Vec3 = Vec3 {
//     x: 0.0,
//     y: 0.0,
//     z: 0.0,
// };
// pub const X3: Vec3 = Vec3 {
//     x: 1.0,
//     y: 0.0,
//     z: 0.0,
// };
// pub const Y3: Vec3 = Vec3 {
//     x: 0.0,
//     y: 1.0,
//     z: 0.0,
// };
// pub const Z3: Vec3 = Vec3 {
//     x: 0.0,
//     y: 0.0,
//     z: 1.0,
// };

// impl Vec3 {
//     pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
//         Vec3 { x, y, z }
//     }

//     pub fn length(&self) -> f32 {
//         (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
//     }

//     pub fn dot(&self, rhs: &Vec3) -> f32 {
//         self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
//     }

//     pub fn cross(&self, rhs: &Vec3) -> Vec3 {
//         let x = self.x;
//         let y = self.y;
//         let z = self.z;
//         let rx = rhs.x;
//         let ry = rhs.y;
//         let rz = rhs.z;

//         Vec3 {
//             x: y * rz - z * ry,
//             y: z * rx - x * rz,
//             z: x * ry - y * rx,
//         }
//     }

//     pub fn unit(&self) -> Vec3 {
//         self / self.length()
//     }

//     pub fn translation_mat(&self) -> Mat4 {
//         Mat4::vec_translate(self)
//     }

//     pub fn scale_mat(&self) -> Mat4 {
//         Mat4::vec_scale(self)
//     }

//     pub fn rotation_mat(&self, radians: Rad) -> Mat4 {
//         Mat4::rotate(radians, self)
//     }

//     pub fn to_homogenous(&self) -> Vec4 {
//         Vec4 {
//             x: self.x,
//             y: self.y,
//             z: self.z,
//             w: 1.0,
//         }
//     }

//     pub fn from_homogenous(h: &Vec4) -> Vec3 {
//         Vec3 {
//             x: h.x / h.w,
//             y: h.y / h.w,
//             z: h.z / h.w,
//         }
//     }

//     // pub fn angle(&self) -> f32 {
//     // self.y.atan2(self.x)
//     // }
// }

// impl ops::Add<Vec3> for Vec3 {
//     type Output = Vec3;

//     fn add(self, rhs: Vec3) -> Self::Output {
//         &self + &rhs
//     }
// }

// impl ops::Add<Vec3> for &Vec3 {
//     type Output = Vec3;

//     fn add(self, rhs: Vec3) -> Self::Output {
//         self + &rhs
//     }
// }

// impl ops::Add<&Vec3> for Vec3 {
//     type Output = Vec3;

//     fn add(self, rhs: &Vec3) -> Self::Output {
//         &self + rhs
//     }
// }

// impl ops::Add<&Vec3> for &Vec3 {
//     type Output = Vec3;

//     fn add(self, rhs: &Vec3) -> Self::Output {
//         Vec3 {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//             z: self.z + rhs.z,
//         }
//     }
// }

// impl ops::Sub<Vec3> for Vec3 {
//     type Output = Vec3;

//     fn sub(self, rhs: Vec3) -> Self::Output {
//         &self - &rhs
//     }
// }

// impl ops::Sub<Vec3> for &Vec3 {
//     type Output = Vec3;

//     fn sub(self, rhs: Vec3) -> Self::Output {
//         self - &rhs
//     }
// }

// impl ops::Sub<&Vec3> for Vec3 {
//     type Output = Vec3;

//     fn sub(self, rhs: &Vec3) -> Self::Output {
//         &self - rhs
//     }
// }

// impl ops::Sub<&Vec3> for &Vec3 {
//     type Output = Vec3;

//     fn sub(self, rhs: &Vec3) -> Self::Output {
//         Vec3 {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//             z: self.z - rhs.z,
//         }
//     }
// }

// impl ops::Mul<f32> for Vec3 {
//     type Output = Vec3;

//     fn mul(self, rhs: f32) -> Self::Output {
//         &self * rhs
//     }
// }

// impl ops::Mul<f32> for &Vec3 {
//     type Output = Vec3;

//     fn mul(self, rhs: f32) -> Self::Output {
//         Vec3 {
//             x: self.x * rhs,
//             y: self.y * rhs,
//             z: self.z * rhs,
//         }
//     }
// }

// impl ops::Mul<Vec3> for f32 {
//     type Output = Vec3;

//     fn mul(self, rhs: Vec3) -> Self::Output {
//         self * &rhs
//     }
// }

// impl ops::Mul<&Vec3> for f32 {
//     type Output = Vec3;

//     fn mul(self, rhs: &Vec3) -> Self::Output {
//         Vec3 {
//             x: self * rhs.x,
//             y: self * rhs.y,
//             z: self * rhs.z,
//         }
//     }
// }

// impl ops::Div<f32> for Vec3 {
//     type Output = Vec3;

//     fn div(self, rhs: f32) -> Self::Output {
//         &self / rhs
//     }
// }

// impl ops::Div<f32> for &Vec3 {
//     type Output = Vec3;

//     fn div(self, rhs: f32) -> Self::Output {
//         Vec3 {
//             x: self.x / rhs,
//             y: self.y / rhs,
//             z: self.z / rhs,
//         }
//     }
// }

// impl ops::Neg for Vec3 {
//     type Output = Vec3;

//     fn neg(self) -> Self::Output {
//         -(&self)
//     }
// }

// impl ops::Neg for &Vec3 {
//     type Output = Vec3;

//     fn neg(self) -> Self::Output {
//         Vec3 {
//             x: -self.x,
//             y: -self.y,
//             z: -self.z,
//         }
//     }
// }

// #[repr(C)]
// #[derive(Clone, Copy, Debug, Default)]
// pub struct Vec4 {
//     pub x: f32,
//     pub y: f32,
//     pub z: f32,
//     pub w: f32,
// }

// pub const ZERO4H: Vec4 = Vec4 {
//     x: 0.0,
//     y: 0.0,
//     z: 0.0,
//     w: 1.0,
// };
// pub const X4H: Vec4 = Vec4 {
//     x: 1.0,
//     y: 0.0,
//     z: 0.0,
//     w: 1.0,
// };
// pub const Y4H: Vec4 = Vec4 {
//     x: 0.0,
//     y: 1.0,
//     z: 0.0,
//     w: 1.0,
// };
// pub const Z4H: Vec4 = Vec4 {
//     x: 0.0,
//     y: 0.0,
//     z: 1.0,
//     w: 1.0,
// };

// impl Vec4 {
//     pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
//         Vec4 { x, y, z, w }
//     }
// }

// impl ops::Add<Vec4> for Vec4 {
//     type Output = Vec4;

//     fn add(self, rhs: Vec4) -> Self::Output {
//         &self + &rhs
//     }
// }

// impl ops::Add<Vec4> for &Vec4 {
//     type Output = Vec4;

//     fn add(self, rhs: Vec4) -> Self::Output {
//         self + &rhs
//     }
// }

// impl ops::Add<&Vec4> for Vec4 {
//     type Output = Vec4;

//     fn add(self, rhs: &Vec4) -> Self::Output {
//         &self + rhs
//     }
// }

// impl ops::Add<&Vec4> for &Vec4 {
//     type Output = Vec4;

//     fn add(self, rhs: &Vec4) -> Self::Output {
//         Vec4 {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//             z: self.z + rhs.z,
//             w: self.w + rhs.w,
//         }
//     }
// }

// impl ops::Sub<Vec4> for Vec4 {
//     type Output = Vec4;

//     fn sub(self, rhs: Vec4) -> Self::Output {
//         &self - &rhs
//     }
// }

// impl ops::Sub<Vec4> for &Vec4 {
//     type Output = Vec4;

//     fn sub(self, rhs: Vec4) -> Self::Output {
//         self - &rhs
//     }
// }

// impl ops::Sub<&Vec4> for Vec4 {
//     type Output = Vec4;

//     fn sub(self, rhs: &Vec4) -> Self::Output {
//         &self - rhs
//     }
// }

// impl ops::Sub<&Vec4> for &Vec4 {
//     type Output = Vec4;

//     fn sub(self, rhs: &Vec4) -> Self::Output {
//         Vec4 {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//             z: self.z - rhs.z,
//             w: self.w - rhs.w,
//         }
//     }
// }

// impl ops::Mul<f32> for Vec4 {
//     type Output = Vec4;

//     fn mul(self, rhs: f32) -> Self::Output {
//         &self * rhs
//     }
// }

// impl ops::Mul<f32> for &Vec4 {
//     type Output = Vec4;

//     fn mul(self, rhs: f32) -> Self::Output {
//         Vec4 {
//             x: self.x * rhs,
//             y: self.y * rhs,
//             z: self.z * rhs,
//             w: self.w * rhs,
//         }
//     }
// }

// impl ops::Mul<Vec4> for f32 {
//     type Output = Vec4;

//     fn mul(self, rhs: Vec4) -> Self::Output {
//         self * &rhs
//     }
// }

// impl ops::Mul<&Vec4> for f32 {
//     type Output = Vec4;

//     fn mul(self, rhs: &Vec4) -> Self::Output {
//         Vec4 {
//             x: self * rhs.x,
//             y: self * rhs.y,
//             z: self * rhs.z,
//             w: self * rhs.w,
//         }
//     }
// }

// impl ops::Div<f32> for Vec4 {
//     type Output = Vec4;

//     fn div(self, rhs: f32) -> Self::Output {
//         &self / rhs
//     }
// }

// impl ops::Div<f32> for &Vec4 {
//     type Output = Vec4;

//     fn div(self, rhs: f32) -> Self::Output {
//         Vec4 {
//             x: self.x / rhs,
//             y: self.y / rhs,
//             z: self.z / rhs,
//             w: self.w / rhs,
//         }
//     }
// }

// impl ops::Neg for Vec4 {
//     type Output = Vec4;

//     fn neg(self) -> Self::Output {
//         -(&self)
//     }
// }

// impl ops::Neg for &Vec4 {
//     type Output = Vec4;

//     fn neg(self) -> Self::Output {
//         Vec4 {
//             x: -self.x,
//             y: -self.y,
//             z: -self.z,
//             w: -self.w,
//         }
//     }
// }
