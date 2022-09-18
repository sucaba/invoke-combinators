use crate::invokes::*;

pub trait NoClosureExt: Sized {
    fn no_closure(self) -> NoClosure<Self> {
        NoClosure(self)
    }
}

impl<I> NoClosureExt for I where I: Iterator {}

pub struct NoClosure<I>(I);

impl<I> NoClosure<I> {
    pub fn map<F>(self, f: F) -> Map<I, F>
    where
        I: Iterator,
        F: InvokeMut<(I::Item,)>,
    {
        Map::new(self.0, f)
    }

    pub fn flat_map<F>(self, f: F) -> FlatMap<I, F>
    where
        I: Iterator,
        F: InvokeMut<(I::Item,)>,
        F::Output: IntoIterator,
    {
        FlatMap::new(self.0, f)
    }
}

pub struct Map<I, F> {
    inner: I,
    map: F,
}

impl<I, F> Map<I, F> {
    pub fn new(inner: I, map: F) -> Self {
        Self { inner, map }
    }
}

impl<I, F, B> Iterator for Map<I, F>
where
    I: Iterator,
    F: InvokeMut<(I::Item,), Output = B>,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|x| self.map.invoke_mut((x,)))
    }
}

pub struct FlatMap<I, F>
where
    I: Iterator,
    F: InvokeMut<(I::Item,)>,
    F::Output: IntoIterator,
{
    inner: I,
    outer: Option<<F::Output as IntoIterator>::IntoIter>,
    f: F,
}

impl<I: Iterator, F> FlatMap<I, F>
where
    I: Iterator,
    F::Output: IntoIterator,
    F: InvokeMut<(I::Item,)>,
{
    pub fn new(inner: I, f: F) -> Self {
        Self {
            inner,
            f,
            outer: None,
        }
    }
}

impl<I, F> Iterator for FlatMap<I, F>
where
    I: Iterator,
    F::Output: IntoIterator,
    F: InvokeMut<(I::Item,)>,
{
    type Item = <F::Output as IntoIterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(outer) = self.outer.as_mut() {
            let result = outer.next();
            if result.is_some() {
                return result;
            }
        }
        while let Some(item) = self.inner.next() {
            let mut outer = self.f.invoke_mut((item,)).into_iter();
            let result = outer.next();
            if result.is_none() {
                continue;
            }
            self.outer = Some(outer);
            return result;
        }

        self.outer = None;
        None
    }
}
