
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

    const fn then<NewFn, O>(self, newfn: NewFn) -> Composer<T, O, impl Fn(T) -> O>
    where
        NewFn: Fn(U) -> O,
    {
        Composer::new(move |input| newfn((self.func)(input)))
    }
    
    // @NOTE: need to wrap this in a move, returning self.f seems to `run destructors...`
    const fn build(self) -> impl Fn(T) -> U {
        move |input| (self.func)(input)
    }
}

pub fn inspect<T: std::fmt::Debug>(item: T) -> T {
    println!("INSPECTING: {:?}", item);
    item
}

const FOOBAR: impl Fn(String) -> i32 = Composer::new(|s: String| s.parse::<i32>())
    .then(|r| r.unwrap())
    .then(inspect)
    .then(|n| n * 2)
    .build();
