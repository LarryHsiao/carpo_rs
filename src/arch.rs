/// Object to fetch data.
pub trait Source<T> {
    /// Fetch the value.
    fn value(&self) -> T;
}

/// Object to manipulate data or do some change.
pub trait Action {
    /// Fire the action.
    fn fire(&self);
}