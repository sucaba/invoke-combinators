use crate::invokes::*;

pub struct RefArg<T>(T);

impl<T> RefArg<T> {
    pub fn new(inner: T) -> Self {
        Self(inner)
    }
}

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
