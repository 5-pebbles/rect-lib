## Unobstructed Rectangle Sweep Line Algorithm

This sweep line algorithm identifies all unique, unobstructed sub-rectangles within a given rectangle by comparing them against a list of obstructions. I was unable to find an efficient algorithm for this problem, so I wrote my own.

**If there is anything that can be improved, please let me know:**

- `Issue:` [GitHub](https://github.com/5-pebbles/rect-lib/issues).
- `Email:` [5-pebble@protonmail.com](mailto:5-pebble@protonmail.com).

### Section 1: Identifying Lines

The sweep line does not traverse every point; it moves from left to right, checking only the points where rectangles transition.

It will identify two types of lines:

1. `Opening Lines:` These appear one unit to the right of each obstruction; this is where a new rectangle may start.
2. `Closing Lines:` These appear on the first unit of each obstruction; this is where a rectangle may end.

There will also be an `opening line` on the first unit of the parent rectangle.

**Take this example:**

![image](https://github.com/5-pebbles/rect-lib/blob/main/explanations/assets/unobstructed_subrectangles-1.svg)

**The algorithm will draw four lines:**

- A `opening line` at the start of the parent rectangle.
- A `closing line` at the start of the first obstruction (same location as the parent's opening line).
- A `opening line` at the end of the first obstruction.
- A `closing line` at the start of the second obstruction.

Lines before or after the parent rectangle, such as the `closing line` of the last obstruction, are discarded.

![image](https://github.com/5-pebbles/rect-lib/blob/main/explanations/assets/unobstructed_subrectangles-2.svg)

### Section 2: Identifying Gaps

For each line, the algorithm identifies gaps between obstructions that may contain rectangles. Obstructions are sorted by their top position, and only those intersecting the current line are considered.

It saves a pointer to the bottom of the last obstruction, which starts at the top of the parent rectangle.

**As it moves down through each obstruction, it:**
- Checks if the last obstruction's bottom is above the current obstruction's top; if it is, there is a gap between them.
- Sets the pointer to the minimum of its current value and the current obstruction's bottom. This is required to handle obstructions that overlap.

**Example of overlapping obstructions:**

![image](https://github.com/5-pebbles/rect-lib/blob/main/explanations/assets/unobstructed_subrectangles-3.svg)

The outer obstruction is processed first; if the pointer is updated to the inner obstruction, a false gap would be found.

### Section 3: Identifying Rectangles

The algorithm maintains a list of active rectangles and a list of completed rectangles.

**processing each line from left to right; here is the logic for each type:**

- `Opening Lines:` For each gap, if not filled by an active rectangle, a new active rectangle is created, starting at the current line with the gap's top and bottom.

![image](https://github.com/5-pebbles/rect-lib/blob/main/explanations/assets/unobstructed_subrectangles-4.svg)

- `Closing Lines:` For each active rectangle, if it fits within a gap, it continues. Otherwise, it is added to the completed rectangles list, ending one unit before the current line. Partially obstructed rectangles are subdivided into the gaps they contain; the new active rectangles have the same start point as the original.

![image](https://github.com/5-pebbles/rect-lib/blob/main/explanations/assets/unobstructed_subrectangles-5.svg)

### Section 4: Finalization

After processing all lines, any remaining active rectangles are added to the completed rectangles list, ending at the parent rectangle's end.

**In our example, this leaves us with four rectangles:**

![image](https://github.com/5-pebbles/rect-lib/blob/main/explanations/assets/unobstructed_subrectangles-6.svg)

### Section 5: Implementation

**Here is the algorithm as I implemented it in the [rect-lib](https://github.com/5-pebbles/rect-lib) crate:**

```rust
use core::cmp::Reverse;
use num::{Num, One};

pub trait Rectangle
where
    Self: Sized + Copy,
{
    // - Required implementations.

    /// The unit type used for the rectangle.
    type Unit: Num + One + Copy + PartialEq + PartialOrd + Ord;

    /// The left most point of the rectangle.
    fn left(&self) -> Self::Unit;

    /// The right most point of the rectangle.
    fn right(&self) -> Self::Unit;

    /// The top most point of the rectangle.
    fn top(&self) -> Self::Unit;

    /// The bottom most point of the rectangle.
    fn bottom(&self) -> Self::Unit;

    /// Creates a new rectangle from the given sides.
    /// The sides are inclusive.
    fn new_from_sides(
        left: Self::Unit,
        right: Self::Unit,
        top: Self::Unit,
        bottom: Self::Unit,
    ) -> Self;

    // - Default implementations.

    /// This algorithm identifies all unique unobstructed sub-rectangles within a given rectangle by comparing it against a list of obstructions.
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
```
