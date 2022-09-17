pub trait InvokeOnce<Args> {
    type Output;

    fn invoke_once(self, args: Args) -> Self::Output;
}

pub trait InvokeMut<Args>: InvokeOnce<Args> {
    fn invoke_mut(&mut self, args: Args) -> Self::Output;
}

pub trait Invoke<Args>: InvokeMut<Args> {
    fn invoke(&self, args: Args) -> Self::Output;
}

pub struct InvokeFn<F>(F);

impl<F, Arg, R> InvokeOnce<(Arg,)> for InvokeFn<F>
where
    F: FnOnce(Arg) -> R,
{
    type Output = F::Output;

    fn invoke_once(self, args: (Arg,)) -> Self::Output {
        self.0(args.0)
    }
}

impl<F, Arg, R> InvokeMut<(Arg,)> for InvokeFn<F>
where
    F: FnMut(Arg) -> R,
{
    fn invoke_mut(&mut self, args: (Arg,)) -> F::Output {
        self.0(args.0)
    }
}

impl<F, Arg, R> Invoke<(Arg,)> for InvokeFn<F>
where
    F: Fn(Arg) -> R,
{
    fn invoke(&self, args: (Arg,)) -> F::Output {
        self.0(args.0)
    }
}

pub struct RefArg<T>(T);

impl<'a, A: 'a + ?Sized, T, R> InvokeOnce<(&&'a A,)> for RefArg<T>
where
    T: InvokeOnce<(&'a A,), Output = R>,
{
    type Output = R;

    fn invoke_once(self, args: (&&'a A,)) -> Self::Output {
        self.0.invoke_once((*args.0,))
    }
}

impl<'a, A: 'a + ?Sized, T> InvokeMut<(&&'a A,)> for RefArg<T>
where
    T: InvokeMut<(&'a A,)>,
{
    fn invoke_mut(&mut self, args: (&&'a A,)) -> Self::Output {
        self.0.invoke_mut((*args.0,))
    }
}

impl<'a, A: 'a + ?Sized, T> Invoke<(&&'a A,)> for RefArg<T>
where
    T: Invoke<(&'a A,)>,
{
    fn invoke(&self, args: (&&'a A,)) -> Self::Output {
        self.0.invoke((*args.0,))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_iter_should_be_iterable() {
        let src = ["red", "green", "blue"];
        let iter: Map<std::slice::Iter<&str>, RefArg<InvokeFn<fn(&'static str) -> String>>> =
            Map::new(src.iter(), RefArg(InvokeFn(String::from)));

        assert_eq!(
            src.into_iter().map(String::from).collect::<Vec<_>>(),
            iter.collect::<Vec<_>>()
        );
    }

    #[test]
    fn invokefn_should_be_invokable() {
        let maplen: InvokeFn<fn(&str) -> usize> = InvokeFn(str::len);
        let len = maplen.invoke(("foobar",));
        assert_eq!(6, len);
    }

    #[test]
    fn flat_map_iter_should_be_iterable() {
        let src: [&'static str; 3] = ["red", "green", "blue"];
        let iter: FlatMap<std::slice::Iter<&str>, RefArg<InvokeFn<fn(&str) -> std::str::Chars>>> =
            FlatMap::new(src.iter(), RefArg(InvokeFn(str::chars)));

        let expected = String::from("redgreenblue");
        let v = iter.collect::<String>();
        assert_eq!(expected, v);
    }
}
