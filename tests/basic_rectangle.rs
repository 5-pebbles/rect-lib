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
fn test_width() {
    let rect = BasicRectangle::new_from_sides(0, 1, 1, 0);
    assert_eq!(rect.width(), 1);

    let rect = BasicRectangle::new_from_sides(0, 4, 3, 0);
    assert_eq!(rect.width(), 4);
}

#[test]
fn test_height() {
    let rect = BasicRectangle::new_from_sides(0, 1, 1, 0);
    assert_eq!(rect.height(), 1);

    let rect = BasicRectangle::new_from_sides(0, 3, 4, 0);
    assert_eq!(rect.height(), 4);
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
fn test_overlaps() {
    let rect1 = BasicRectangle::new_from_sides(0, 1, 1, 0);
    let rect2 = BasicRectangle::new_from_sides(2, 3, 3, 2);
    assert!(!rect1.overlaps(&rect2));

    let rect1 = BasicRectangle::new_from_sides(0, 1, 1, 0);
    let rect2 = BasicRectangle::new_from_sides(1, 2, 2, 1);
    assert!(rect1.overlaps(&rect2));
}

#[test]
fn test_contains_point() {
    let rect = BasicRectangle::new_from_sides(0, 2, 2, 0);
    assert!(rect.contains_point(1, 1));
    assert!(rect.contains_point(0, 0));
    assert!(rect.contains_point(0, 2));

    assert!(!rect.contains_point(3, 3));
}

#[test]
fn test_intersection_no_overlap() {
    let rect1 = BasicRectangle::new_from_sides(0, 1, 1, 0);
    let rect2 = BasicRectangle::new_from_sides(2, 3, 3, 2);
    let intersection = rect1.intersection(&rect2);
    assert!(intersection.is_none());
}

#[test]
fn test_intersection_overlap() {
    let rect1 = BasicRectangle::new_from_sides(0, 1, 1, 0);
    let rect2 = BasicRectangle::new_from_sides(1, 2, 2, 1);
    let intersection = rect1.intersection(&rect2).expect("Rectangles do not overlap");
    assert_eq!(intersection.left(), 1);
    assert_eq!(intersection.right(), 1);
    assert_eq!(intersection.top(), 1);
    assert_eq!(intersection.bottom(), 1);
}

#[test]
fn test_contains_rectangle() {
    let rect1 = BasicRectangle::new_from_sides(0, 2, 2, 0);
    let rect2 = BasicRectangle::new_from_sides(1, 2, 2, 1);
    assert!(rect1.contains_rectangle(&rect2));
    assert!(!rect2.contains_rectangle(&rect1));
}
