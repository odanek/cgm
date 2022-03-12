use crate::{Mat2, Mat3, Mat4, Vec2, Vec3, Vec4};

macro_rules! easy_impl {
    ( $( $std_name:ident $imp_ty:ty { $($field:ident),* }, )* ) => {
        $(
            impl crevice::std140::AsStd140 for $imp_ty {
                type Output = crevice::std140::$std_name;

                #[inline]
                #[allow(clippy::needless_update)]
                fn as_std140(&self) -> Self::Output {
                    crevice::std140::$std_name {
                        $(
                            $field: self.$field.as_std140(),
                        )*
                        ..bytemuck::Zeroable::zeroed()
                    }
                }

                #[inline]
                fn from_std140(value: Self::Output) -> Self {
                    Self {
                        $(
                            $field: <_ as crevice::std140::AsStd140>::from_std140(value.$field),
                        )*
                    }
                }
            }
        )*
    };
}

easy_impl! {
    Vec2 Vec2<f32> { x, y },
    Vec3 Vec3<f32> { x, y, z },
    Vec4 Vec4<f32> { x, y, z, w },

    DVec2 Vec2<f64> { x, y },
    DVec3 Vec3<f64> { x, y, z },
    DVec4 Vec4<f64> { x, y, z, w },

    Mat2 Mat2<f32> { x, y },
    Mat3 Mat3<f32> { x, y, z },
    Mat4 Mat4<f32> { x, y, z, w },

    DMat2 Mat2<f64> { x, y },
    DMat3 Mat3<f64> { x, y, z },
    DMat4 Mat4<f64> { x, y, z, w },
}
