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

pub struct ConstSource<T: Copy> {
    pub _value: T,
}

impl<T: Copy + PartialEq> Source<T> for ConstSource<T> {
    fn value(&self) -> Result<T, Box<dyn Error>> {
        Ok(self._value)
    }
}

impl<T: Copy + PartialEq> PartialEq for ConstSource<T> {
    fn eq(&self, other: &Self) -> bool {
        self._value == other._value
    }
}
