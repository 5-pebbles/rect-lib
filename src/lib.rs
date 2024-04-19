mod basic_rectangle;
pub use basic_rectangle::BasicRectangle;

pub trait Rectangle<T> {
    fn left(&self) -> T;
    fn right(&self) -> T;
    fn top(&self) -> T;
    fn bottom(&self) -> T;
    fn from_sides(left: T, right: T, top: T, bottom: T) -> Self;
}
