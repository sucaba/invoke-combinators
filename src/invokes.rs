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

impl<F> InvokeFn<F> {
    pub fn new(f: F) -> Self {
        Self(f)
    }
}

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
