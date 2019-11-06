use std::error::Error;

/// Object to fetch data.
pub trait Source<T> {
    /// Fetch the value.
    fn value(&self) -> Result<T, Box<dyn Error>>;
}

/// Object to manipulate data or do some change.
pub trait Action {
    /// Fire the action.
    fn fire(&self) -> Result<(), Box<dyn Error>>;
}