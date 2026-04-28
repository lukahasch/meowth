#[derive(Clone, Copy, Debug)]
pub struct Arguments<T>(T);

#[derive(Clone, Copy, Debug)]
pub struct Single<T> {
    item: T,
}

#[derive(Clone, Copy, Debug)]
pub struct Car<T, O> {
    item: T,
    rest: O,
}

pub struct Here;
pub struct There<I>(I);

pub trait Access<T, W> {
    fn access(&self) -> &T;
}

impl<T> Access<T, Here> for Single<T> {
    fn access(&self) -> &T {
        &self.item
    }
}

impl<T, O> Access<T, Here> for Car<T, O> {
    fn access(&self) -> &T {
        &self.item
    }
}

impl<T, O, B, I> Access<B, There<I>> for Car<T, O>
where
    O: Access<B, I>,
{
    fn access(&self) -> &B {
        self.rest.access()
    }
}

impl<T> Arguments<Single<T>> {
    pub fn new(item: T) -> Self {
        Arguments(Single { item })
    }
}

impl Arguments<()> {
    pub fn empty() -> Self {
        Arguments(())
    }
}

impl<T> Arguments<T> {
    pub fn push<O>(self, item: O) -> Arguments<Car<O, T>> {
        Arguments(Car { item, rest: self.0 })
    }

    pub fn access<O, I>(&self) -> &O
    where
        T: Access<O, I>,
    {
        self.0.access()
    }
}

impl<T, O> Arguments<Car<O, T>> {
    pub fn pop(self) -> (O, Arguments<T>) {
        let Car { item, rest } = self.0;
        (item, Arguments(rest))
    }
}

impl<T, B, I> Access<B, There<I>> for Arguments<T>
where
    T: Access<B, I>,
{
    fn access(&self) -> &B {
        self.0.access()
    }
}

pub fn test() {
    let arguments = Arguments::empty().push(1).push("test");
    dbg!(arguments);
    dbg!(arguments.access::<&str, _>());
}
