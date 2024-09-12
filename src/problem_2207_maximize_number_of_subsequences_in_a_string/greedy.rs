pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        let [left, right]: [_; 2] = pattern.into_bytes().try_into().ok().unwrap();
        let mut left_count = 0_u64;
        let mut right_count = 0_u64;
        let mut result = 0;

        for c in text.into_bytes() {
            result += left_count * u64::from(c == right);
            left_count += u64::from(c == left);
            right_count += u64::from(c == right);
        }

        (result + left_count.max(right_count)) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn maximum_subsequence_count(text: String, pattern: String) -> i64 {
        Self::maximum_subsequence_count(text, pattern)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
