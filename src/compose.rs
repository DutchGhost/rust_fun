use std::marker::PhantomData;

pub struct Composer<T, U, F> {
    func: F,
    marker: PhantomData<fn(T) -> U>,
}

impl<T, U, F> Composer<T, U, F> {
    #[inline(always)]
    pub const fn new(func: F) -> Self {
        Self {
            func,
            marker: PhantomData,
        }
    }

    #[inline(always)]
    const fn then<Closure, Output>(self, f: Closure) -> Composer<T, Output, impl Fn(T) -> Output>
    where
        F: Fn(T) -> U,
        Closure: Fn(U) -> Output,
    {
        Composer::new(move |input| f((self.func)(input)))
    }

    #[inline(always)]
    const fn then_mut<Closure, Output>(
        mut self,
        mut f: Closure,
    ) -> Composer<T, Output, impl FnMut(T) -> Output>
    where
        F: FnMut(T) -> U,
        Closure: FnMut(U) -> Output,
    {
        Composer::new(move |input| f((self.func)(input)))
    }

    #[inline(always)]
    const fn then_once<Closure, Output>(
        self,
        f: Closure,
    ) -> Composer<T, Output, impl FnOnce(T) -> Output>
    where
        F: Fn(T) -> U,
        Closure: FnOnce(U) -> Output,
    {
        Composer::new(move |input| f((self.func)(input)))
    }

    // @NOTE: need to wrap this in a move, returning self.f seems to `run destructors...`
    #[inline(always)]
    const fn build(self) -> impl Fn(T) -> U
    where
        F: Fn(T) -> U,
    {
        move |input| (self.func)(input)
    }

    #[inline(always)]
    const fn build_mut(mut self) -> impl FnMut(T) -> U
    where
        F: FnMut(T) -> U,
    {
        move |input| (self.func)(input)
    }

    #[inline(always)]
    const fn build_once(self) -> impl FnOnce(T) -> U
    where
        F: FnOnce(T) -> U,
    {
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
