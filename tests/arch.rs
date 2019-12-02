use carpo::arch::{ConstSource, Source};

/// Check the result of ConstSource::value() with primitive type int .
#[test]
fn const_source_int() {
    assert_eq!(ConstSource { _value: 12 }.value().unwrap(), 12)
}

/// Check the result of ConstSource::value() with primitive type c_str.
#[test]
fn const_source_c_str() {
    assert_eq!(ConstSource { _value: "abc" }.value().unwrap(), "abc")
}

/// Check the result of ConstSource::value() with ConstSource.
#[test]
fn const_source_equal() {
    let reference = ConstSource { _value: 0 }; // Obj ref
    assert!(ConstSource { _value: &reference }.value().unwrap() == &reference)
}

/// Check the result of ConstSource::value() with ConstSource negative case.
#[test]
fn const_source_not_equal() {
    let reference = ConstSource { _value: 0 }; // Obj ref
    let reference2 = ConstSource { _value: 1 }; // Obj ref
    assert!(ConstSource { _value: &reference }.value().unwrap() != &reference2)
}

/// Check the result of ConstSource::value() with two same value ConstSource.
#[test]
fn const_source_not_equal_two_obj() {
    let reference = ConstSource { _value: 0 }; // Obj ref
    let reference2 = ConstSource { _value: 0 }; // Obj ref
    assert!(ConstSource { _value: &reference }.value().unwrap() == &reference2)
}
