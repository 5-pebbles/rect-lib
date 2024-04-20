use rect_lib::{BasicRectangle, Rectangle};

#[test]
fn test_basic_rectangle() {
    let rect = BasicRectangle::new_from_sides(0, 1, 2, 3);
    assert_eq!(rect.left(), 0);
    assert_eq!(rect.right(), 1);
    assert_eq!(rect.top(), 2);
    assert_eq!(rect.bottom(), 3);
}

#[test]
fn test_perimeter_rectangle() {
    let rect = BasicRectangle::new_from_sides(0, 1, 1, 0);
    assert_eq!(rect.perimeter(), 4);
}
