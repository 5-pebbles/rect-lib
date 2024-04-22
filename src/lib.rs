use core::cmp::Reverse;
use num::{Num, One};

// re-export the num crate
pub use num;

// basic rectangle
mod basic_rectangle;
pub use basic_rectangle::BasicRectangle;

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
        /// A rectangle that has not been obstructed yet
        #[derive(Clone)]
        struct UnfinishedRect<T: Rectangle> {
            left: T::Unit,
            top: T::Unit,
            bottom: T::Unit,
        }
        /// A gap between two obstructions
        struct Gap<T: Rectangle> {
            top: T::Unit,
            bottom: T::Unit,
        }
        /// A line we need to check for gaps
        struct Line<T: Rectangle> {
            x: T::Unit,
            opens: bool,
        }

        let mut obstructions = obstructions.to_vec();
        // sort the obstructions by top position
        obstructions.sort_unstable_by(
            // descending order
            |rect_a, rect_b| {
                rect_b.top().cmp(&rect_a.top()) // by the first point on each
            },
        );

        // Section 1: collect all lines that need to be checked for gaps
        let mut lines: Vec<Line<Self>> = vec![Line {
            x: self.left(),
            opens: true,
        }];

        for rect in &obstructions {
            // gaps might close on the left of each obstruction
            lines.push(Line {
                x: rect.left(),
                opens: false,
            });

            // gaps might open just after the right of each obstruction
            lines.push(Line {
                x: rect.right() + Self::Unit::one(),
                opens: true,
            });
        }

        // order from left to right
        lines.sort_unstable_by_key(|line| line.x);
        lines.dedup_by_key(|line| line.x);

        // filter out lines that are outside the rectangle
        let lines = lines
            .into_iter()
            .filter(|line| self.left() <= line.x && line.x <= self.right());

        // this is the list we will return
        let mut unique_rectangles: Vec<Self> = Vec::new();

        // this will store active rectangles as we sweep from line to line
        let mut active_rectangles: Vec<UnfinishedRect<Self>> = Vec::new();

        for line in lines {
            // Section 2: collect all gaps between obstructions
            let mut gaps: Vec<Gap<Self>> = Vec::new();

            // think of each obstruction as a shingle on a roof
            // if the bottom of one shingle is above the top of the next there is a gap between them
            let mut last_rectange_bottom: Self::Unit = self.top();

            // filter out obstructions that don't intersect the current line
            for obstruction in obstructions
                .iter()
                .filter(|rect| rect.left() <= line.x && line.x <= rect.right())
            {
                if last_rectange_bottom > obstruction.top() {
                    gaps.push(Gap {
                        top: last_rectange_bottom,
                        bottom: obstruction.top() + Self::Unit::one(), // the top is inclusive so +1
                    });
                }

                // if a later shingle starts in the same place we could get a fake gap
                // so we avoid that by getting the lowest point
                last_rectange_bottom =
                    last_rectange_bottom.min(obstruction.bottom() - Self::Unit::one());
            }

            // check if there is a gap between the bottom of the last shingle and the end of the roof
            // the bottom is inclusive so >=
            if last_rectange_bottom >= self.bottom() {
                gaps.push(Gap {
                    top: last_rectange_bottom,
                    bottom: self.bottom(),
                });
            }
            // alright, we have all the gaps

            active_rectangles.sort_unstable_by_key(|rect| Reverse(rect.left));

            // Section 3: if the current line opens we create new rectangles
            if line.opens {
                // try to create a new rect for each gap
                for gap in gaps {
                    // make sure its unique
                    if !active_rectangles
                        .iter()
                        .any(|rect| gap.top == rect.top && gap.bottom == rect.bottom)
                    {
                        active_rectangles.push(UnfinishedRect {
                            left: line.x,
                            top: gap.top,
                            bottom: gap.bottom,
                        });
                    }
                }

                // on to the next line
                continue;
            }

            // Section 3 & 1/2: if the current line closes we finish rectangles
            let mut new_active_rectangles: Vec<UnfinishedRect<Self>> = Vec::new();

            active_rectangles = active_rectangles
                .iter()
                .filter(|rect| {
                    // if the current rect fits within a gap we can keep it
                    for gap in gaps.iter() {
                        if gap.top >= rect.top && rect.bottom >= gap.bottom {
                            // on to the next active rect
                            return true;
                        }
                    }

                    // if it is obstructed we can close it
                    unique_rectangles.push(Self::new_from_sides(
                        rect.left,                  // left
                        line.x - Self::Unit::one(), // right
                        rect.top,                   // top
                        rect.bottom,                // bottom
                    ));

                    // check if there are any gaps within the current rect
                    for gap in gaps
                        .iter()
                        .filter(|gap| gap.top <= rect.top || rect.bottom <= gap.bottom)
                    {
                        let top_limit = rect.top.min(gap.top);
                        let bottom_limit = rect.bottom.max(gap.bottom);

                        // make sure its unique
                        if !active_rectangles
                            .iter()
                            .chain(new_active_rectangles.iter())
                            .any(|rect| top_limit == rect.top && bottom_limit == rect.bottom)
                        {
                            new_active_rectangles.push(UnfinishedRect {
                                left: rect.left,
                                top: top_limit,
                                bottom: bottom_limit,
                            });
                        }
                    }

                    // make sure to remove it from active
                    false
                })
                .cloned()
                .collect();

            // add any new sub rectangles
            active_rectangles.append(&mut new_active_rectangles);
        }

        // Section 4: now that we have checked all lines we can close any remaining rectangles
        for rect in active_rectangles {
            unique_rectangles.push(Self::new_from_sides(
                rect.left,
                self.right(),
                rect.top,
                rect.bottom,
            ));
        }

        // Quod Erat Demonstrandum
        unique_rectangles
    }
}
