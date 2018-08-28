use std::marker::PhantomData;
use super::*;

// struct FnCompose<T, O, F>
// {
//     func: F,
//     _items: PhantomData<(T, O)>,
// }

// impl <T, O, F> FnCompose<T, O, F> {
//     const fn new(func: F) -> Self {
//         Self {
//             func,
//             _items: PhantomData,
//         }
//     }

//     const fn then<NewO, F2>(self, func: F2) -> FnCompose<T, NewO, impl Fn(T) -> NewO>
//     where
//         F: Fn(T) -> O,
//         F2: Fn(O) -> NewO
//     {
//         FnCompose {
//             func: move |t| func((self.func)(t)),
//             _items: PhantomData
//         }
//     }
// }

// struct FnCompose<T, M, O, F1, F2>
// {
//     func: Composed<T, M, O, F1, F2>,
//     _items: PhantomData<(T, M, O)>,
// }

// existential type tester<T1, T2, T3, F1, F2>: Fn(T1) -> T3;

// impl <T, M, O, F1, F2> FnCompose<T, M, O, F1, F2>
// where
//     F1: Fn(T) -> M,
//     F2: Fn(M) -> O,
// {
//     const fn new(func: F1) -> FnCompose<T, M, M, F1, impl Fn(M) -> M>
//     {
//         FnCompose {
//             func: compose(func, |input| input),
//             _items: PhantomData,
//         }
//     }

//     fn then<NewO, F3: Fn(O) -> NewO>(self, func: F3) -> FnCompose<T, O, NewO, impl Fn(T) -> O, impl Fn(O) -> NewO>
//     where
//     {
//         FnCompose {
            
//             func: compose(self.func, func),
//             _items: PhantomData
//         }
//     }

//     fn into_inner(self) -> tester<T, M, O, F1, F2> {
//         self.func
//     }
// }

// pub existential type Funcy<T1, O, T2, F, F2>: Fn(T1) -> T2;

// pub struct FnCompose<F>
// {
//     func: F,
// }

// impl <F> FnCompose<F>
// {
//     const fn new(func: F) -> FnCompose<F>
//     {
//         FnCompose {
//             func: func,
//         }
//     }

//     const fn then<T, O, NewO, F2: Fn(O) -> NewO>(self, func: F2) -> FnCompose<Funcy<T, O, NewO, F, F2>>
//     where
//         F: Fn(T) -> O 
//     {
//         FnCompose {
//             func: move |input| func((self.func)(input)),
//         }
//     }

// }

// macro_rules! funcs {
//     ($T: ty, $F: ty, $F2: ty) => {
//         Funcy<$T, $T, $T, $F, $F2>
//     }
// }

struct FnCompose<F>
{
    func: F,
}

impl <F> FnCompose<F>
{
    const fn new(func: F) -> FnCompose<F>
    {
        FnCompose {
            func: func,
        }
    }

    const fn then<T, O, NewO, F2: Fn(O) -> NewO>(self, func: F2) -> FnCompose<impl Fn(T) -> NewO>
    where
        F: Fn(T) -> O,
    {
        FnCompose {
            func: move |input| func((self.func)(input))
        }
    }

    const fn into_inner<T, O>(self) -> impl Fn(T) -> O
    where
        F: Fn(T) -> O,
    {
        move |input| (self.func)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    existential type Funcy<T, U>: Fn(T) -> U;

    const fn happy<T, U>(composed: FnCompose<impl Fn(T) -> U>) -> Funcy<T, U> {
        composed.into_inner()
    }

    const fooz: Funcy<usize, usize> = happy(FnCompose::new(|x| x).then(|x| x + 1));
}