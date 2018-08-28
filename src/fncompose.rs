use std::marker::PhantomData;
use super::*;

struct FnCompose<T, O, F: Fn(T) -> O>
{
    func: F,
    _items: PhantomData<(T, O)>,
}

impl <T, O, F: Fn(T) -> O> FnCompose<T, O, F> {
    const fn new(func: F) -> Self {
        Self {
            func,
            _items: PhantomData,
        }
    }

    const fn then<OO, F2: Fn(O) -> OO>(self, func: F2) -> FnCompose<T, O, F> {
        Self {
            func: compose(self.func, func) as _,
            _items: PhantomData
        }
    }
}