use crate::Rectangle;

pub struct BasicRectangle {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rectangle<i32> for BasicRectangle {
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

    fn from_sides(left: i32, right: i32, top: i32, bottom: i32) -> Self {
        BasicRectangle {
            x: left,
            y: top,
            width: right - left,
            height: top - bottom,
        }
    }
}

