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

pub existential type Funcy<T1, T2, F: Fn(T1) -> T2>: Fn(T1) -> T2;

struct FnCompose<T, O, F>
{
    func: F,
    _items: PhantomData<(T, O)>,
}

impl <T, O, F> FnCompose<T, O, F>
where
    F: Fn(T) -> O,
{
    const fn new(func: F) -> FnCompose<T, O, F>
    {
        FnCompose {
            func: func,
            _items: PhantomData,
        }
    }

    const fn then<NewO, F2: Fn(O) -> NewO>(self, func: F2) -> FnCompose<T, NewO, impl Fn(T) -> NewO>
    where
    {
        FnCompose {
            func: move |input| func((self.func)(input)),
            _items: PhantomData
        }
    }

    const fn into_inner(self) -> Funcy<T, O, F> {
        self.func
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
    
//     #[test]
//     fn test() {
//         let SuperFn = FnCompose::<usize, usize, usize, fn(usize) -> usize, fn(usize) -> usize>::new(|x| x + 0)
//         .then(|x| x + 1)
//         .then(|x| x + 2)
//         .then(|x| x + 3).into_inner();

//         assert_eq!(SuperFn(10), 16);
//     }
// }