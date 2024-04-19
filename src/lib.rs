use core::cmp::Reverse;
use num::{Num, One};

// re-export the num crate
pub use num;

// basic rectangle
mod basic_rectangle;
pub use basic_rectangle::BasicRectangle;

// & here's our trait
pub trait Rectangle
where
    Self: Sized + Copy,
{
    // - Required implementations.

    /// The unit type used for the rectangle.
    type Unit: Num + One + Copy + PartialEq + PartialOrd + Ord;

    /// The left side of the rectangle.
    fn left(&self) -> Self::Unit;

    /// The right side of the rectangle.
    fn right(&self) -> Self::Unit;

    /// The top side of the rectangle.
    fn top(&self) -> Self::Unit;

    /// The bottom side of the rectangle.
    fn bottom(&self) -> Self::Unit;

    /// Creates a new rectangle from the given sides.
    fn new_from_sides(
        left: Self::Unit,
        right: Self::Unit,
        top: Self::Unit,
        bottom: Self::Unit,
    ) -> Self;


    // - Default implementations.

    /// The width of the rectangle.
    fn width(&self) -> Self::Unit {
        self.right() - self.left()
    }

    /// The height of the rectangle.
    fn height(&self) -> Self::Unit {
        self.top() - self.bottom()
    }

    /// The area of the rectangle.
    fn area(&self) -> Self::Unit {
        // This function is so cute for some reason
        self.width() * self.height() // :3
    }

    /// Checks if the rectangle contains the given point.
    fn contains_point(&self, x: Self::Unit, y: Self::Unit) -> bool {
        x >= self.left() && x <= self.right() && y >= self.top() && y <= self.bottom()
    }

    /// Checks if one rectangle contains another.
    fn contains_rectangle<T>(&self, other: &T) -> bool
    where
        T: Rectangle<Unit = Self::Unit>,
    {
        self.left() <= other.left()
            && self.right() >= other.right()
            && self.top() >= other.top()
            && self.bottom() <= other.bottom()
    }

    /// Checks if one rectangle overlaps with another.
    fn overlaps<T>(&self, other: &T) -> bool
    where
        T: Rectangle<Unit = Self::Unit>,
    {
        self.left() < other.right()
            && self.right() > other.left()
            && self.top() > other.bottom()
            && self.bottom() < other.top()
    }

    /// Returns the intersection of two rectangles.
    fn intersection<T>(&self, other: &T) -> Option<Self>
    where
        T: Rectangle<Unit = Self::Unit>,
    {
        let left = self.left().max(other.left());
        let right = self.right().min(other.right());
        let top = self.top().min(other.top());
        let bottom = self.bottom().max(other.bottom());

        if left < right && bottom < top {
            Some(Self::new_from_sides(left, right, top, bottom))
        } else {
            None
        }
    }

    /// This algorithm identifies all unique unobstructed sub-rectangles within a given rectangle by comparing it against a list of obstructions.
    fn unobstructed_subrectangles<T, U>(&self, obstructions: &[&T]) -> Vec<Self>
    where
        T: Rectangle<Unit = Self::Unit>,
    {
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

        // collect all lines that need to be checked for gaps
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
            // Section 1: collect all gaps between obstructions
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
                        bottom: obstruction.top() + Self::Unit::one(),
                    });
                }

                // if the current shingle starts a the same place but is shorter
                // then we could have a fake gap
                last_rectange_bottom =
                    last_rectange_bottom.min(obstruction.bottom() - Self::Unit::one());
            }

            // check if there is a gap between the bottom of the last shingle and the end of the roof
            if last_rectange_bottom > self.bottom() {
                gaps.push(Gap {
                    top: last_rectange_bottom,
                    bottom: self.bottom(),
                });
            }
            // alright, we have all the gaps

            active_rectangles.sort_unstable_by_key(|rect| Reverse(rect.left));

            // Section 2: if the current line opens we create new rectangles
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

            // Section 3: if the current line closes we finish rectangles
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

        // now that we have checked all lines we can close any remaining rectangles
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
