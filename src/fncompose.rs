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

    const fn then<OO>(self, func: impl Fn(O) -> OO) -> FnCompose<T, OO, impl Fn(T) -> OO>
    where
        F: Fn(T) -> O,
    {
        Self {
            func: compose(self.func, func),
            _items: PhantomData
        }
    }
}