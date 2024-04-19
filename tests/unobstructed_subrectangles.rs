use rect_lib::{BasicRectangle, Rectangle};

#[test]
fn test_unobstructed_subrectangles_no_obstructions() {
    let rect = BasicRectangle::new_from_sides(0, 1, 2, 0);
    let obstructions: Vec<&BasicRectangle> = Vec::new();
    let subrects = rect.unobstructed_subrectangles(&obstructions);
    assert_eq!(subrects.len(), 1);
    assert_eq!(subrects[0].left(), 0);
    assert_eq!(subrects[0].right(), 1);
    assert_eq!(subrects[0].top(), 2);
    assert_eq!(subrects[0].bottom(), 0);
}

#[test]
fn test_unobstructed_subrectangles_fully_obstructed() {
    let rect = BasicRectangle::new_from_sides(0, 1, 2, 0);
    let obstructions = vec![&rect];
    let subrects = rect.unobstructed_subrectangles(&obstructions);
    assert_eq!(subrects.len(), 0);
}

#[test]
fn test_unobstructed_subrectangles_part_obstructed() {
    let rect = BasicRectangle::new_from_sides(0, 5, 5, 0);
    let obstruction = BasicRectangle::new_from_sides(0, 2, 5, 1);
    let subrects = rect.unobstructed_subrectangles(&vec![&obstruction]);
    assert_eq!(subrects.len(), 2);
    // there should be one along the bottom edge
    assert!(subrects.contains(&BasicRectangle::new_from_sides(0, 5, 0, 0)));
    // & one at the end
    assert!(subrects.contains(&BasicRectangle::new_from_sides(3, 5, 5, 0)));
}
