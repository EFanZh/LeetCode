pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut ones = 0;
        let mut prev = 0;
        let mut result = 0;

        for c in s.into_bytes() {
            if c != b'0' {
                if prev == b'0' {
                    result += ones;
                }

                ones += 1;
            }

            prev = c;
        }

        if prev == b'0' {
            result += ones;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_operations(s: String) -> i32 {
        Self::max_operations(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
