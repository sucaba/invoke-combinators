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

impl<F, Arg, R> InvokeOnce<(Arg,)> for F
where
    F: FnOnce(Arg) -> R,
{
    type Output = F::Output;

    fn invoke_once(self, args: (Arg,)) -> Self::Output {
        self(args.0)
    }
}

impl<F, Arg, R> InvokeMut<(Arg,)> for F
where
    F: FnMut(Arg) -> R,
{
    fn invoke_mut(&mut self, args: (Arg,)) -> F::Output {
        self(args.0)
    }
}

impl<F, Arg, R> Invoke<(Arg,)> for F
where
    F: Fn(Arg) -> R,
{
    fn invoke(&self, args: (Arg,)) -> F::Output {
        self(args.0)
    }
}
