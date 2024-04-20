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

#[test]
fn test_area_rectangle() {
    let rect = BasicRectangle::new_from_sides(0, 1, 1, 0);
    assert_eq!(rect.area(), 1);

    let rect = BasicRectangle::new_from_sides(0, 4, 4, 0);
    assert_eq!(rect.area(), 16);
}

#[test]
fn test_contains_point() {
    let rect = BasicRectangle::new_from_sides(0, 2, 2, 0);
    assert!(rect.contains_point(1, 1));
    assert!(rect.contains_point(0, 0));
    assert!(rect.contains_point(0, 2));

    assert!(!rect.contains_point(3, 3));
}
