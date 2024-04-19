use crate::Rectangle;

/// A basic rectangle implementation.
/// Edges are inclusive.
#[derive(Clone, Copy, Debug)]
pub struct BasicRectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rectangle for BasicRectangle {
    type Unit = i32;

    fn left(&self) -> i32 {
        self.x
    }

    fn right(&self) -> i32 {
        self.x + self.width - 1
    }

    fn top(&self) -> i32 {
        self.y
    }

    fn bottom(&self) -> i32 {
        self.y - self.height + 1
    }

    fn new_from_sides(left: i32, right: i32, top: i32, bottom: i32) -> Self {
        BasicRectangle {
            x: left,
            y: top,
            width: right - left + 1,
            height: top - bottom + 1,
        }
    }
}
