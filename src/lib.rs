use num::{Num, One};

// re-export the num crate
pub use num;

// basic rectangle
mod basic_rectangle;
pub use basic_rectangle::BasicRectangle;
use unobstructed_sweep_line::unobstructed_subrectangles_impl;

mod unobstructed_sweep_line;

/// A trait containing methods for rectangle like data structures which implement `Sized` & `Copy`.
///
/// This trait treats all edges (left, right, top, & bottom) as inclusive.
///
/// # Example
/// ```
/// use rect_lib::Rectangle;
///
/// #[derive(Clone, Copy)]
/// pub struct BasicRectangle {
///     x: i32,
///     y: i32,
///     width: i32,
///     height: i32,
/// }
///
/// impl Rectangle for BasicRectangle {
///     type Unit = i32;
///
///     fn left(&self) -> i32 {
///         self.x
///     }
///
///     fn right(&self) -> i32 {
///         self.x + self.width - 1
///     }
///
///     fn top(&self) -> i32 {
///         self.y
///     }
///
///     fn bottom(&self) -> i32 {
///         self.y - self.height + 1
///     }
///
///     fn new_from_sides(left: i32, right: i32, top: i32, bottom: i32) -> Self {
///         Self {
///             x: left,
///             y: top,
///             width: right - left + 1,
///             height: top - bottom + 1,
///         }
///     }
/// }
/// ```
pub trait Rectangle
where
    Self: Sized + Copy,
{
    // - Required implementations.

    /// The unit type used for the rectangle.
    type Unit: Num + One + Copy + PartialEq + PartialOrd + Ord;

    /// The left most point of the rectangle.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 1, 2, 3);
    /// assert_eq!(rect.left(), 0);
    /// ````
    fn left(&self) -> Self::Unit;

    /// The right most point of the rectangle.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 1, 2, 3);
    /// assert_eq!(rect.right(), 1);
    /// ```
    fn right(&self) -> Self::Unit;

    /// The top most point of the rectangle.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 1, 2, 3);
    /// assert_eq!(rect.top(), 2);
    /// ```
    fn top(&self) -> Self::Unit;

    /// The bottom most point of the rectangle.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 1, 2, 3);
    /// assert_eq!(rect.bottom(), 3);
    /// ```
    fn bottom(&self) -> Self::Unit;

    /// Creates a new rectangle from the given sides.
    /// The sides are inclusive.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 1, 2, 3);
    /// ```
    fn new_from_sides(
        left: Self::Unit,
        right: Self::Unit,
        top: Self::Unit,
        bottom: Self::Unit,
    ) -> Self;

    // - Default implementations.

    /// The width of the rectangle.
    /// This is calculated as `right - left`.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 1, 1, 0);
    /// assert_eq!(rect.width(), 1);
    /// ```
    fn width(&self) -> Self::Unit {
        self.right() - self.left()
    }

    /// The height of the rectangle.
    /// This is calculated as `top - bottom`.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 1, 1, 0);
    /// assert_eq!(rect.height(), 1);
    /// ```
    fn height(&self) -> Self::Unit {
        self.top() - self.bottom()
    }

    /// Translates the rectangle by the given amount.
    /// This is done by adding the given amount to the x and y coordinates.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 1, 1, 0);
    /// let translated = rect.translate(1, 1);
    /// assert_eq!(translated, BasicRectangle::new_from_sides(1, 2, 2, 1));
    /// ```
    fn translate(&self, x: Self::Unit, y: Self::Unit) -> Self {
        Self::new_from_sides(
            self.left() + x,
            self.right() + x,
            self.top() + y,
            self.bottom() + y,
        )
    }

    /// The perimeter of the rectangle.
    /// This is calculated as `(width + height) * 2`.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 1, 1, 0);
    /// assert_eq!(rect.perimeter(), 4);
    /// ```
    fn perimeter(&self) -> Self::Unit {
        (self.width() + self.height()) * (Self::Unit::one() + Self::Unit::one())
    }

    /// The area of the rectangle.
    /// This is calculated as `width * height`.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 1, 1, 0);
    /// assert_eq!(rect.area(), 1);
    /// ```
    fn area(&self) -> Self::Unit {
        // This function is so cute for some reason
        self.width() * self.height() // :3
    }

    /// Checks if the rectangle contains the given point.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 1, 1, 0);
    /// assert!(rect.contains_point(0, 1));
    /// assert!(!rect.contains_point(0, 2));
    /// ```
    fn contains_point(&self, x: Self::Unit, y: Self::Unit) -> bool {
        x >= self.left() && x <= self.right() && y <= self.top() && y >= self.bottom()
    }

    /// Checks if one rectangle contains another.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 2, 2, 0);
    /// let other = BasicRectangle::new_from_sides(0, 1, 1, 0);
    /// assert!(rect.contains_rectangle(&other));
    /// assert!(!other.contains_rectangle(&rect));
    /// ```
    fn contains_rectangle(&self, other: &impl Rectangle<Unit = Self::Unit>) -> bool {
        self.left() <= other.left()
            && self.right() >= other.right()
            && self.top() >= other.top()
            && self.bottom() <= other.bottom()
    }

    /// Checks if one rectangle overlaps with another.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 2, 2, 0);
    /// assert!(rect.overlaps(&BasicRectangle::new_from_sides(1, 3, 3, 1)));
    /// assert!(!rect.overlaps(&BasicRectangle::new_from_sides(3, 4, 4, 3)));
    /// ```
    fn overlaps(&self, other: &impl Rectangle<Unit = Self::Unit>) -> bool {
        self.left() <= other.right()
            && self.right() >= other.left()
            && self.top() >= other.bottom()
            && self.bottom() <= other.top()
    }

    /// Returns the intersection of two rectangles.
    /// If the rectangles do not intersect, `None` is returned.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 2, 2, 0);
    /// let intersection = rect.intersection(&BasicRectangle::new_from_sides(1, 3, 3, 1));
    /// assert_eq!(intersection, Some(BasicRectangle::new_from_sides(1, 2, 2, 1)));
    ///
    /// let no_intersection = rect.intersection(&BasicRectangle::new_from_sides(3, 4, 4, 3));
    /// assert_eq!(no_intersection, None);
    /// ```
    fn intersection(&self, other: &impl Rectangle<Unit = Self::Unit>) -> Option<Self> {
        let left = self.left().max(other.left());
        let right = self.right().min(other.right());
        let top = self.top().min(other.top());
        let bottom = self.bottom().max(other.bottom());

        if left <= right && bottom <= top {
            Some(Self::new_from_sides(left, right, top, bottom))
        } else {
            None
        }
    }

    /// This algorithm identifies all unique unobstructed sub-rectangles within a given rectangle by comparing it against a list of obstructions.
    ///
    /// # Example
    /// ```
    /// use rect_lib::{BasicRectangle, Rectangle};
    ///
    /// let rect = BasicRectangle::new_from_sides(0, 5, 5, 0);
    /// let obstruction = BasicRectangle::new_from_sides(0, 2, 5, 1);
    /// let subrects = rect.unobstructed_subrectangles(&vec![&obstruction]);
    ///
    /// assert_eq!(subrects.len(), 2);
    /// assert!(subrects.iter().all(|r| [
    ///     BasicRectangle::new_from_sides(0, 5, 0, 0),
    ///     BasicRectangle::new_from_sides(3, 5, 5, 0)
    /// ].contains(r)));
    /// ```
    fn unobstructed_subrectangles(
        &self,
        obstructions: &[&impl Rectangle<Unit = Self::Unit>],
    ) -> Vec<Self> {
        unobstructed_subrectangles_impl(self, obstructions)
    }
}
