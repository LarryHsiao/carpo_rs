use carpo::arch::*;
use carpo::util::*;

#[test]
fn simple() {
    assert!(IsImage {
        file_name: "image.png".to_string()
    }
    .value()
    .unwrap());


    assert!(IsImage {
        file_name: "image.jpg".to_string()
    }
        .value()
        .unwrap());


    assert!(!IsImage {
        file_name: "file.whatever".to_string()
    }
        .value()
        .unwrap());
}
