pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut result = 0;
        let mut length = 0;
        let mut prev = 0;

        for c in s.into_bytes() {
            if c == prev {
                length += 1;
            } else {
                length = 1;
                prev = c;
            }

            result = result.max(length);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_power(s: String) -> i32 {
        Self::max_power(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
