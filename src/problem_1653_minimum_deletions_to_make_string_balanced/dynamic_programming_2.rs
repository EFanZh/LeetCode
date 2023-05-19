pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut result = 0_u32;
        let mut b_count = 0_u32;

        for c in s.bytes() {
            if c == b'a' {
                result = b_count.min(result + 1);
            } else {
                b_count += 1;
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_deletions(s: String) -> i32 {
        Self::minimum_deletions(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
