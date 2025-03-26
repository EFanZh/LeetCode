pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn capture_forts(forts: Vec<i32>) -> i32 {
        let mut iter = forts.iter().copied();

        iter.find(|&x| x != 0).map_or(0, |mut left| {
            let mut result = 0_u32;
            let mut length = 0_u32;

            for c in iter {
                if c == 0 {
                    length += 1;
                } else {
                    if c != left {
                        left = c;
                        result = result.max(length);
                    }

                    length = 0;
                }
            }

            result
        }) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn capture_forts(forts: Vec<i32>) -> i32 {
        Self::capture_forts(forts)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
