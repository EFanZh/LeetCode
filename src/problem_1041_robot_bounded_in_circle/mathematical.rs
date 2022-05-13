pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_robot_bounded(s: String) -> bool {
        let mut location = (0, 0);
        let mut direction = (0_i8, 1_i8);

        for c in s.into_bytes() {
            match c {
                b'G' => {
                    location.0 += direction.0;
                    location.1 += direction.1;
                }
                b'L' => direction = (-direction.1, direction.0),
                _ => direction = (direction.1, -direction.0),
            }
        }

        location == (0, 0) || direction != (0, 1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_robot_bounded(s: String) -> bool {
        Self::is_robot_bounded(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
