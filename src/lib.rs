#![feature(const_fn)]
#![feature(existential_type)]

/// A type decleration for the [`compose`] function.
/// This *has* to carry around the F1 and F2 generic parameters, which stand for the 2 function being composed.
pub existential type Composed<T1, T2, T3, F1, F2>: Fn(T1) -> T3;

/// Composes 2 function's into a new function.
/// This is powerfull, because it can be done at compiletime,
/// and therefore you can assign the composed function to a constant decleration.
/// # Examples
/// ```
/// use rust_fun::{compose, Composed};
/// 
/// // helps seperating the types and the decleration
/// type Example = Composed<usize, usize, usize, fn(usize) -> usize, fn(usize) -> usize>;
/// const CONST_CLOSURE: Example = compose(|n| n * 2, |n| n * 3);
/// assert_eq!(CONST_CLOSURE(10), 60);
/// ```
pub const fn compose<T1, T2, T3, F1, F2>(func1: F1, func2: F2) -> Composed<T1, T2, T3, F1, F2>
where
    F1: Fn(T1) -> T2,
    F2: Fn(T2) -> T3
{
    move |input| func2(func1(input))
}

use std::sync::{Arc, Mutex};
pub const ARCED_MUTEX_USIZE: Composed<usize, Mutex<usize>, Arc<Mutex<usize>>, fn(usize) -> Mutex<usize>, fn(Mutex<usize>) -> Arc<Mutex<usize>>> =     compose(Mutex::new, Arc::new);

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

        type Example = Composed<usize, usize, usize, fn(usize) -> usize, fn(usize) -> usize>;

        const ADD_ONE_THEN_SQUARE: Example = compose(add_one, square);
        const SQUARE_THEN_ADD_ONE: Example = compose(square, add_one);

        assert_eq!(ADD_ONE_THEN_SQUARE(10), 22);
        assert_eq!(SQUARE_THEN_ADD_ONE(10), 21);

        const CONST_CLOSURE: Example = compose(|n| n * 2, |n| n * 3);

        assert_eq!(CONST_CLOSURE(10), 60);
    }
}