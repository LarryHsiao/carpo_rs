pub trait Source<T> {
    fn value(&self) -> T;
}
