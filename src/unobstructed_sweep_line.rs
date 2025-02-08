use core::cmp::Reverse;
use num::{Num, One};

use crate::Rectangle;

pub fn unobstructed_subrectangles_impl<Unit, Parent>(
    parent: &Parent,
    obstructions: &[&impl Rectangle<Unit = Unit>],
) -> Vec<Parent>
where
    Unit: Num + One + Copy + PartialEq + PartialOrd + Ord,
    Parent: Rectangle<Unit = Unit>,
{
    /// A rectangle that has not been obstructed yet.
    #[derive(Clone)]
    struct UnfinishedRect<T: Rectangle> {
        left: T::Unit, // Start
        top: T::Unit,
        bottom: T::Unit,
    }
    /// A gap between two obstructions.
    struct Gap<T: Rectangle> {
        top: T::Unit,
        bottom: T::Unit,
    }
    /// A line we need to check for gaps.
    struct Line<T: Rectangle> {
        x: T::Unit,
        opens: bool,
    }

    let mut obstructions = obstructions.to_vec();
    // Sort the obstructions:
    obstructions.sort_unstable_by(
        // Descending order by the first point on each.
        |rect_a, rect_b| rect_b.top().cmp(&rect_a.top()),
    );

    // Section 1: Collect all lines that need to be checked for gaps.
    let mut lines: Vec<Line<Parent>> = vec![Line {
        x: parent.left(),
        opens: true,
    }];

    for rect in &obstructions {
        // Gaps might close on the left of each obstruction:
        lines.push(Line {
            x: rect.left(),
            opens: false,
        });

        // Gaps might open just after the right of each obstruction:
        lines.push(Line {
            x: rect.right() + Unit::one(),
            opens: true,
        });
    }

    // Order from left to right:
    lines.sort_unstable_by_key(|line| line.x);
    lines.dedup_by_key(|line| line.x);

    // Filter out lines outside the rectangle:
    let lines = lines
        .into_iter()
        .filter(|line| parent.left() <= line.x && line.x <= parent.right());

    // This is the list our function will return:
    let mut unique_rectangles: Vec<Parent> = Vec::new();

    // This will store active rectangles as we sweep between lines:
    let mut active_rectangles: Vec<UnfinishedRect<Parent>> = Vec::new();

    for line in lines {
        // Section 2: Collect all gaps between obstructions.
        let mut gaps: Vec<Gap<Parent>> = Vec::new();

        // Think of each obstruction as a shingle on a roof:
        // If the bottom of one shingle is above the top of the next there is a gap between them.
        let mut last_rectange_bottom: Unit = parent.top();

        // Filter out obstructions that don't intersect the current line.
        for obstruction in obstructions
            .iter()
            .filter(|rect| rect.left() <= line.x && line.x <= rect.right())
        {
            if last_rectange_bottom > obstruction.top() {
                gaps.push(Gap {
                    top: last_rectange_bottom,
                    bottom: obstruction.top() + Unit::one(), // NOTE: The top is inclusive so +1.
                });
            }

            // If a later shingle starts in the same place we could get a fake gap.
            // We avoid that by getting the lowest point.
            last_rectange_bottom = last_rectange_bottom.min(obstruction.bottom() - Unit::one());
        }

        // Check if there is a gap between the bottom of the last shingle and the end of the roof.
        // The bottom is inclusive so >=...
        if last_rectange_bottom >= parent.bottom() {
            gaps.push(Gap {
                top: last_rectange_bottom,
                bottom: parent.bottom(),
            });
        }
        // Alright, we have all the gaps...

        active_rectangles.sort_unstable_by_key(|rect| Reverse(rect.left));

        // Section 3: If the current line opens we create new rectangles.
        if line.opens {
            // Try to create a new rect for each gap.
            for gap in gaps {
                // Make sure its unique.
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

            // On to the next line...
            continue;
        }

        // Section 3 & 1/2: If the current line closes we finish rectangles.
        let mut new_active_rectangles: Vec<UnfinishedRect<Parent>> = Vec::new();

        active_rectangles = active_rectangles
            .iter()
            .filter(|rect| {
                // If the current rect fits within a gap we can keep it.
                if gaps
                    .iter()
                    .any(|gap| gap.top >= rect.top && rect.bottom >= gap.bottom)
                {
                    // On to the next active rect...
                    return true;
                }

                // If it's obstructed we close it.
                unique_rectangles.push(Parent::new_from_sides(
                    rect.left,            // left
                    line.x - Unit::one(), // right
                    rect.top,             // top
                    rect.bottom,          // bottom
                ));

                // Check if there are any gaps within the current rect.
                for gap in gaps
                    .iter()
                    .filter(|gap| gap.top <= rect.top || rect.bottom <= gap.bottom)
                {
                    let top_limit = rect.top.min(gap.top);
                    let bottom_limit = rect.bottom.max(gap.bottom);

                    // Confirm it's unique.
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

                // Make sure we remove it from active.
                false
            })
            .cloned()
            .collect();

        // Add any new sub rectangles.
        active_rectangles.append(&mut new_active_rectangles);
    }

    // Section 4: Now that we have checked all lines we can close any remaining rectangles.
    for rect in active_rectangles {
        unique_rectangles.push(Parent::new_from_sides(
            rect.left,
            parent.right(),
            rect.top,
            rect.bottom,
        ));
    }

    // Quod Erat Demonstrandum
    unique_rectangles
}
