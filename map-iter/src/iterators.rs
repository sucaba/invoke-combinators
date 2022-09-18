use crate::invokes::*;

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

pub struct FlatMap<I: Iterator, F: Invoke<(I::Item,)>>
where
    F::Output: IntoIterator,
{
    inner: I,
    outer: Option<<F::Output as IntoIterator>::IntoIter>,
    f: F,
}

impl<I: Iterator, F: Invoke<(I::Item,)>> FlatMap<I, F>
where
    F::Output: IntoIterator,
{
    pub fn new(inner: I, f: F) -> Self {
        Self {
            inner,
            f,
            outer: None,
        }
    }
}

impl<I, F: Invoke<(I::Item,)>> Iterator for FlatMap<I, F>
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
            let mut outer = self.f.invoke((item,)).into_iter();
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
