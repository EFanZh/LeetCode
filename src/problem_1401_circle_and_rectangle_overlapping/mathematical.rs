pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    #[allow(clippy::never_loop)] // Not supported by LeetCode.
    pub fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        let (x, y) = loop {
            let distance = if y_center < y1 {
                if x_center < x1 {
                    // Bottom left.

                    break (x1, y1);
                } else if x_center < x2 {
                    // Bottom.

                    y1 - y_center
                } else {
                    // Bottom right.

                    break (x2, y1);
                }
            } else if y_center < y2 {
                if x_center < x1 {
                    // Left.

                    x1 - x_center
                } else if x_center < x2 {
                    // Center.

                    return true;
                } else {
                    // Right.

                    x_center - x2
                }
            } else if x_center < x1 {
                // Top left.

                break (x1, y2);
            } else if x_center < x2 {
                // Top.

                y_center - y2
            } else {
                // Top right.

                break (x2, y2);
            };

            return distance <= radius;
        };

        (x_center - x).pow(2) + (y_center - y).pow(2) <= radius.pow(2)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_overlap(radius: i32, x_center: i32, y_center: i32, x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
        Self::check_overlap(radius, x_center, y_center, x1, y1, x2, y2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
