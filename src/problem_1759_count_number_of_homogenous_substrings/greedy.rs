pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        let mut prev = 0;
        let mut length = 0;
        let mut result = 0_u64;

        for c in s.bytes() {
            if c == prev {
                length += 1;
            } else {
                result += length * (length + 1) / 2;
                prev = c;
                length = 1;
            }
        }

        result += length * (length + 1) / 2;

        (result % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_homogenous(s: String) -> i32 {
        Self::count_homogenous(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
