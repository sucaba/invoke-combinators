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

pub struct ToStringMapper;

impl<'a> InvokeOnce<(&'a str,)> for ToStringMapper {
    type Output = String;

    fn invoke_once(self, args: (&'a str,)) -> Self::Output {
        String::from(args.0)
    }
}

impl<'a> InvokeMut<(&'a str,)> for ToStringMapper {
    fn invoke_mut(&mut self, args: (&'a str,)) -> Self::Output {
        String::from(args.0)
    }
}

impl<'a> Invoke<(&'a str,)> for ToStringMapper {
    fn invoke(&self, args: (&'a str,)) -> Self::Output {
        String::from(args.0)
    }
}

pub struct RefArg<T>(T);

impl<'a, A: 'a + ?Sized, T, R> InvokeOnce<(&&'a A,)> for RefArg<T>
where
    T: InvokeOnce<(&'a A,), Output = R>,
{
    type Output = R;

    fn invoke_once(self, args: (&&'a A,)) -> Self::Output {
        let a: &'a A = args.0;
        self.0.invoke_once((a,))
    }
}

impl<'a, A: 'a + ?Sized, T> InvokeMut<(&&'a A,)> for RefArg<T>
where
    T: InvokeMut<(&'a A,)>,
{
    fn invoke_mut(&mut self, args: (&&'a A,)) -> Self::Output {
        let a: &'a A = args.0;
        self.0.invoke_mut((a,))
    }
}

impl<'a, A: 'a + ?Sized, T> Invoke<(&&'a A,)> for RefArg<T>
where
    T: Invoke<(&'a A,)>,
{
    fn invoke(&self, args: (&&'a A,)) -> Self::Output {
        let a: &'a A = args.0;
        self.0.invoke((a,))
    }
}

pub struct MapIter<I, F> {
    inner: I,
    map: F,
}

impl<I, F> MapIter<I, F> {
    pub fn new(inner: I, map: F) -> Self {
        Self { inner, map }
    }
}

impl<I, F, B> Iterator for MapIter<I, F>
where
    I: Iterator,
    F: InvokeMut<(I::Item,), Output = B>,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|x| self.map.invoke_mut((x,)))
    }
}

pub struct FlatMapIter<I: Iterator, F: Invoke<(I::Item,)>>
where
    F::Output: IntoIterator,
{
    inner: I,
    outer_iter: <F::Output as IntoIterator>::Item,
    flat_map: F,
}

impl<I: Iterator, F: Invoke<(I::Item,)>> FlatMapIter<I, F>
where
    F::Output: IntoIterator,
{
    pub fn new(inner: I, flat_map: F) -> Self {
        Self {
            inner,
            outer_iter,
            flat_map,
        }
    }
}

impl<I, F, B> Iterator for FlatMapIter<I, F>
where
    I: Iterator,
    B: IntoIterator,
    F: InvokeMut<(I::Item,), Output = B>,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.flat_map.invoke_mut((x,))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let src = vec!["red", "green", "blue"];
        let iter = MapIter::new(src.iter(), RefArg(ToStringMapper));

        for s in iter {
            println!("color={s}");
        }
    }
}
