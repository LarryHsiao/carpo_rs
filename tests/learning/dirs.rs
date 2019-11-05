use std::path::{PathBuf, Path};

/// learning test for dirs crate.
#[test]
fn home_is_dir() {
    assert!(dirs::home_dir().unwrap().is_dir())
}