use std::marker::PhantomData;
use super::*;

struct FnCompose<T, O, F>
{
    func: F,
    _items: PhantomData<(T, O)>,
}

impl <T, O, F> FnCompose<T, O, F> {
    const fn new(func: F) -> Self {
        Self {
            func,
            _items: PhantomData,
        }
    }

    const fn then<NewO, F2>(self, func: F2) -> FnCompose<T, NewO, impl Fn(T) -> NewO>
    where
        F: Fn(T) -> O,
        F2: Fn(O) -> NewO
    {
        FnCompose {
            func: compose(self.func, func),
            _items: PhantomData
        }
    }
}