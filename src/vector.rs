use std::ops::{Add, Sub, Mul, Div, Neg};

use crate::num::{Num, One, Signed, Zero};

pub trait Vec 
// where
//     Self : Index<usize, Output = <Self as Vec>::Element>,
//     Self : IndexMut<usize, Output = <Self as Vec>::Element>
{
    type Element: Copy;

    fn len() -> usize;
    // TODO product and sum
}

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

        impl<S: Copy> Vec for $VecN<S> {
            type Element = S;

            fn len() -> usize {
                $n
            }
        }

        impl_operator!(<S: Num>, Add<$VecN<S>>, $VecN<S>, {
            fn add(lhs, rhs) -> $VecN<S> { $VecN { $($field: lhs.$field + rhs.$field),+ } }
        });
        impl_operator!(<S: Num>, Sub<$VecN<S>>, $VecN<S>, {
            fn sub(lhs, rhs) -> $VecN<S> { $VecN { $($field: lhs.$field - rhs.$field),+ } }
        });
        impl_operator!(<S: Num>, Mul<S>, $VecN<S>, {
            fn mul(lhs, rhs) -> $VecN<S> { $VecN { $($field: lhs.$field * rhs),+ } }
        });
        impl_operator!(<S: Num>, Div<S>, $VecN<S>, {
            fn div(lhs, rhs) -> $VecN<S> { $VecN { $($field: lhs.$field / rhs),+ } }
        });
        impl_scalar_ops!($VecN<usize> { $($field),+ });
        impl_scalar_ops!($VecN<u8> { $($field),+ });
        impl_scalar_ops!($VecN<u16> { $($field),+ });
        impl_scalar_ops!($VecN<u32> { $($field),+ });
        impl_scalar_ops!($VecN<u64> { $($field),+ });
        impl_scalar_ops!($VecN<isize> { $($field),+ });
        impl_scalar_ops!($VecN<i8> { $($field),+ });
        impl_scalar_ops!($VecN<i16> { $($field),+ });
        impl_scalar_ops!($VecN<i32> { $($field),+ });
        impl_scalar_ops!($VecN<i64> { $($field),+ });
        impl_scalar_ops!($VecN<f32> { $($field),+ });
        impl_scalar_ops!($VecN<f64> { $($field),+ });

        impl<S: Signed> Neg for $VecN<S> {
            type Output = $VecN<S>;

            fn neg(self) -> Self::Output {
                $VecN { $($field: -self.$field),+ }
            }
        }

        impl<'a, S: Signed> Neg for &'a $VecN<S> {
            type Output = $VecN<S>;

            fn neg(self) -> Self::Output {
                $VecN { $($field: -self.$field),+ }
            }
        }
    }
}

macro_rules! impl_scalar_ops {
    ($VecN:ident<$S:ident> { $($field:ident),+ }) => {
        impl_operator!(Mul<$VecN<$S>>, $S, {
            fn mul(scalar, vector) -> $VecN<$S> { $VecN { $($field: scalar * vector.$field),+ } }
        });
        impl_operator!(Div<$VecN<$S>>, $S, {
            fn div(scalar, vector) -> $VecN<$S> { $VecN { $($field: scalar / vector.$field),+ } }
        });        
    };
}

impl_vector!(Vec1 { x }, 1);
impl_vector!(Vec2 { x, y }, 2);
impl_vector!(Vec3 { x, y, z }, 3);
impl_vector!(Vec4 { x, y, z, w }, 4);


impl<S: One> Vec1<S> {
    const X: Vec1<S> = Vec1 { x: <S as One>::ONE };
}

impl<S: One + Zero> Vec2<S> {
    const X: Vec2<S> = Vec2 {
        x: <S as One>::ONE,
        y: <S as Zero>::ZERO,
    };
    const Y: Vec2<S> = Vec2 {
        x: <S as Zero>::ZERO,
        y: <S as One>::ONE,
    };
}

impl<S: One + Zero> Vec3<S> {
    const X: Vec3<S> = Vec3 {
        x: <S as One>::ONE,
        y: <S as Zero>::ZERO,
        z: <S as Zero>::ZERO,
    };
    const Y: Vec3<S> = Vec3 {
        x: <S as Zero>::ZERO,
        y: <S as One>::ONE,
        z: <S as Zero>::ZERO,
    };
    const Z: Vec3<S> = Vec3 {
        x: <S as Zero>::ZERO,
        y: <S as Zero>::ZERO,
        z: <S as One>::ONE,
    };
}

impl<S: One + Zero> Vec4<S> {
    const X: Vec4<S> = Vec4 {
        x: <S as One>::ONE,
        y: <S as Zero>::ZERO,
        z: <S as Zero>::ZERO,
        w: <S as One>::ONE,
    };
    const Y: Vec4<S> = Vec4 {
        x: <S as Zero>::ZERO,
        y: <S as One>::ONE,
        z: <S as Zero>::ZERO,
        w: <S as One>::ONE,
    };
    const Z: Vec4<S> = Vec4 {
        x: <S as Zero>::ZERO,
        y: <S as Zero>::ZERO,
        z: <S as One>::ONE,
        w: <S as One>::ONE,
    };
    const W: Vec4<S> = Vec4 {
        x: <S as Zero>::ZERO,
        y: <S as Zero>::ZERO,
        z: <S as Zero>::ZERO,
        w: <S as One>::ONE,
    };
}

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


// impl Vec3 {
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

#[cfg(test)]
mod tests {
    mod vec {

    }

    mod vec2 {

    }

    mod vec3 {

    }

    mod vec4 {
        
    }
}