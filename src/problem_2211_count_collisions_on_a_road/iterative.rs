pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let mut result = 0;
        let mut iter = directions.bytes();

        for c in iter.by_ref() {
            if c != b'L' {
                let mut count = u32::from(c == b'R');

                for c in iter {
                    if c == b'R' {
                        count += 1;
                    } else {
                        result += count + u32::from(c == b'L');

                        count = 0;
                    }
                }

                break;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_collisions(directions: String) -> i32 {
        Self::count_collisions(directions)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
