#![feature(const_fn)]
#![feature(existential_type)]

pub mod fncompose;

macro_rules! doc_comment {
    ($x:expr, $($tt:tt)*) => {
        #[doc = $x]
        $($tt)*
    };
}

macro_rules! meta_compose {
    ($fnname: ident, $functype: tt, $ext_type: tt, $($mutifier:tt)*) => (
        doc_comment!(
            concat!("A type declaration for the [`", stringify!($fnname), "`] function"),
            pub existential type $ext_type<T1, T2, T3, F1, F2>: $functype(T1) -> T3;
        );

        doc_comment!(
            concat!("
                Composes 2 function's into a new function.
                This is powerfull, because it can be done at compiletime,
                and therefore you can assign the composed function to a constant variable.
                # Examples

                ```
                use rust_fun::{", stringify!($fnname), ", ", stringify!($ext_type), "};

                type Example = ", stringify!($ext_type), "<usize, usize, usize, fn(usize) -> usize, fn(usize) -> usize>;
                const CONST_CLOSURE: Example = compose(|n| n * 2, |n| n * 3);
                assert_eq!(CONST_CLOSURE(10), 60);
                ```
            "),
            pub const fn $fnname<T1, T2, T3, F1, F2>($($mutifier)* func1: F1, $($mutifier)* func2: F2) -> $ext_type<T1, T2, T3, F1, F2>
            where
                F1: $functype(T1) -> T2,
                F2: $functype(T2) -> T3,
            {
                move |input| func2(func1(input))
            }
        );
    )
}

meta_compose!(compose, Fn, Composed,);
meta_compose!(compose_mut, FnMut, ComposedMut, mut);
meta_compose!(compose_once, FnOnce, ComposedOnce,);

/// This macro Takes a 'Composed' type, and 3 types.
/// The 3 types are the Input/Output types of the Function's being composed.
#[macro_export]
macro_rules! Comped {
    ($C:tt, $t1:ty, $t2:ty, $t3:ty) => {
        $C<$t1, $t2, $t3, fn($t1) -> $t2, fn($t2) -> $t3>
    };

    ($C:tt, $t1:ty) => {
        $C<$t1, $t1, $t1, fn($t1) -> $t1, fn($t1) -> $t1>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compose() {
        use std::ops::{Add, Mul};

        use super::{Composed, compose};

        fn square<T: Mul<usize, Output = T>>(n: T) -> T {
            n * 2
        }

        fn add_one<T: Add<usize, Output = T>>(n: T) -> T {
            n + 1
        }

        type Example = Comped!(Composed, usize); //Composed<usize, usize, usize, fn(usize) -> usize, fn(usize) -> usize>;

        const ADD_ONE_THEN_SQUARE: Example = compose(add_one, square);
        const SQUARE_THEN_ADD_ONE: Example = compose(square, add_one);

        assert_eq!(ADD_ONE_THEN_SQUARE(10), 22);
        assert_eq!(SQUARE_THEN_ADD_ONE(10), 21);

        const CONST_CLOSURE: Example = compose(|n| n * 2, |n| n * 3);

        assert_eq!(CONST_CLOSURE(10), 60);
    }
}
