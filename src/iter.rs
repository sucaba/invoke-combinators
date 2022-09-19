use crate::invokes::*;

pub trait IteratorInvokeExt: Iterator + Sized {
    fn map_invoke<F>(self, f: F) -> MapInvoke<Self, F>
    where
        F: InvokeMut<(Self::Item,)>,
    {
        MapInvoke::new(self, f)
    }

    fn flat_map_invoke<F>(self, f: F) -> FlatMapInvoke<Self, F>
    where
        F: InvokeMut<(Self::Item,)>,
        F::Output: IntoIterator,
    {
        FlatMapInvoke::new(self, f)
    }
}

impl<I> IteratorInvokeExt for I where I: Iterator {}

pub struct MapInvoke<I, F> {
    inner: I,
    map: F,
}

impl<I, F> MapInvoke<I, F> {
    pub fn new(inner: I, map: F) -> Self {
        Self { inner, map }
    }
}

impl<I, F, B> Iterator for MapInvoke<I, F>
where
    I: Iterator,
    F: InvokeMut<(I::Item,), Output = B>,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|x| self.map.invoke_mut((x,)))
    }
}

pub struct FlatMapInvoke<I, F>
where
    I: Iterator,
    F: InvokeMut<(I::Item,)>,
    F::Output: IntoIterator,
{
    inner: I,
    outer: Option<<F::Output as IntoIterator>::IntoIter>,
    f: F,
}

impl<I: Iterator, F> FlatMapInvoke<I, F>
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

impl<I, F> Iterator for FlatMapInvoke<I, F>
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
