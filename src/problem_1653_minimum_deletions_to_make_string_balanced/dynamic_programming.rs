pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut min_deletions_a = 0_u32;
        let mut min_deletions_b = 0_u32;

        for c in s.bytes() {
            if c == b'a' {
                min_deletions_b += 1;
            } else {
                min_deletions_b = min_deletions_b.min(min_deletions_a);
                min_deletions_a += 1;
            }
        }

        min_deletions_a.min(min_deletions_b) as _
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
