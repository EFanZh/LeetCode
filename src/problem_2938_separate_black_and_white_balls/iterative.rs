pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut blacks = 0_u64;
        let mut result = 0_u64;

        for c in s.bytes() {
            if c == b'0' {
                result += blacks;
            } else {
                blacks += 1;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_steps(s: String) -> i64 {
        Self::minimum_steps(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
