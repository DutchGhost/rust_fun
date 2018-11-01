
use std::marker::PhantomData;

pub struct Composer<T, U, F> {
    func: F,
    marker: PhantomData<fn(T) -> U>,
}

impl<T, U, F> Composer<T, U, F>
where
    F: Fn(T) -> U,
{
    pub const fn new(func: F) -> Self {
        Self {
            func,
            marker: PhantomData,
        }
    }

    const fn then<Closure, Output>(self, f: Closure) -> Composer<T, Output, impl Fn(T) -> Output>
    where
        Closure: Fn(U) -> Output,
    {
        Composer::new(move |input| f((self.func)(input)))
    }

    const fn then_mut<Closure, Output>(self, f: Closure) -> Composer<T, Output, impl Fn(T) -> Output>
    where
        Closure: FnMut(U) -> Output
    {
        Composer::new(move |mut input| f((self.func)(input)))
    }

    const fn then_once<Closure, Output>(self, f: Closure) -> Composer<T, Output, impl Fn(T) -> Output>
    where
        Closure: FnOnce(U) -> Output
    {
        Composer::new(move |input| f((self.func)(input)))
    }

    // @NOTE: need to wrap this in a move, returning self.f seems to `run destructors...`
    const fn build(self) -> impl Fn(T) -> U {
        move |input| (self.func)(input)
    }

    const fn build_mut(self) -> impl FnMut(T) -> U {
        move |input| (self.func)(input)
    }

    const fn build_once(self) -> impl FnOnce(T) -> U {
        move |input| (self.func)(input)
    }
}

pub fn inspect<T: std::fmt::Debug>(item: T) -> T {
    println!("INSPECTING: {:?}", item);
    item
}

const FOOBAR: impl FnOnce(String) -> i32 = Composer::new(|s: String| s.parse::<i32>())
    .then(|r| r.unwrap())
    .then(inspect)
    .then(|n| n * 2)
    .build_once();
