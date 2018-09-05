use std::marker::PhantomData;

pub struct FnCompose<T, O, F>
{
    func: F,

    marker: PhantomData<fn(T) -> O>
}

impl <T, O, F> FnCompose<T, O, F>
where
    F: Fn(T) -> O
{
    pub const fn new(func: F) -> FnCompose<T, O, F>
    {
        FnCompose {
            func: func,
            marker: PhantomData,
        }
    }

    pub const fn then<NewO, F2: Fn(O) -> NewO>(self, func: F2) -> FnCompose<T, NewO, impl Fn(T) -> NewO>
    {
        FnCompose {
            func: move |input| func((self.func)(input)),
            marker: PhantomData
        }
    }

    pub const fn into_inner(self) -> impl Fn(T) -> O
    {
        move |input| (self.func)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    existential type Parse: Fn(String) -> usize;

    const fn make_parser() -> Parse {
        FnCompose::new(|s: String| s.parse()).then(|r| r.unwrap_or(0)).into_inner()
    }

    const FOOZ: Parse = make_parser();

    #[test]
    fn test_fooz() {
        assert_eq!(FOOZ(String::from("10")), 10);
    }
}
