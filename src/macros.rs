macro_rules! impl_operator {
    // Unary operator
    (<$S:ident: $Constraint:ident> $Op:ident for $Lhs:ty {
        fn $op:ident($x:ident) -> $Output:ty { $body:expr }
    }) => {
        impl<$S: $Constraint> $Op for $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self) -> $Output {
                let $x = self;
                $body
            }
        }

        impl<'a, $S: $Constraint> $Op for &'a $Lhs {
            type Output = $Output;
            #[inline]
            fn $op(self) -> $Output {
                let $x = self;
                $body
            }
        }
    };
    // Right-hand side is a scalar
    (<$S:ident: $Constraint:ident> $Op:ident<$Rhs:ident> for $Lhs:ty {
        fn $op:ident($lhs: ident, $rhs: ident) -> $Out:ty { $body:expr }
    }) => {
        impl<$S: $Constraint> $Op<$Rhs> for $Lhs {
            type Output = $Out;

            #[inline]
            fn $op(self, other: $Rhs) -> Self::Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a, $S: $Constraint> $Op<$Rhs> for &'a $Lhs {
            type Output = $Out;

            #[inline]
            fn $op(self, other: $Rhs) -> Self::Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }
    };
    // Right hand side is a type
    (<$S:ident: $Constraint:ident> $Op:ident<$Rhs:ty> for $Lhs:ty {
        fn $op:ident($lhs: ident, $rhs: ident) -> $Out:ty { $body:expr }
    }) => {
        impl<$S: $Constraint> $Op<$Rhs> for $Lhs {
            type Output = $Out;

            #[inline]
            fn $op(self, other: $Rhs) -> Self::Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a, $S: $Constraint> $Op<$Rhs> for &'a $Lhs {
            type Output = $Out;

            #[inline]
            fn $op(self, other: $Rhs) -> Self::Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a, $S: $Constraint> $Op<&'a $Rhs> for $Lhs {
            type Output = $Out;

            #[inline]
            fn $op(self, other: &'a $Rhs) -> Self::Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a, 'b, $S: $Constraint> $Op<&'a $Rhs> for &'b $Lhs {
            type Output = $Out;

            #[inline]
            fn $op(self, other: &'a $Rhs) -> Self::Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }
    };
    // When the left operand is a scalar
    ($Op:ident<$Rhs:ident<$S:ident>> for $Lhs:ty {
        fn $op:ident($lhs: ident, $rhs: ident) -> $Out:ty { $body:expr }
    }) => {
        impl $Op<$Rhs<$S>> for $Lhs {
            type Output = $Out;

            #[inline]
            fn $op(self, other: $Rhs<$S>) -> Self::Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }

        impl<'a> $Op<&'a $Rhs<$S>> for $Lhs {
            type Output = $Out;

            #[inline]
            fn $op(self, other: &'a $Rhs<$S>) -> Self::Output {
                let ($lhs, $rhs) = (self, other);
                $body
            }
        }
    };
}

macro_rules! impl_assignment_operator {
    (<$S:ident: $Constraint:ident> $Op:ident<$Rhs:ty> for $Lhs:ty {
        fn $op:ident(&mut $lhs:ident, $rhs:ident) $body:block
    }) => {
        impl<$S: $Constraint + $Op<$S>> $Op<$Rhs> for $Lhs {
            #[inline]
            fn $op(&mut $lhs, $rhs: $Rhs) $body
        }
    };
}
