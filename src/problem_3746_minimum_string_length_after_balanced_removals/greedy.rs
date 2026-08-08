pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_length_after_removals(s: String) -> i32 {
        let sum = s.bytes().map(u32::from).sum::<u32>();
        let n = s.len() as u32;
        let b_count = sum - u32::from(b'a') * n;
        let a_count = n - b_count;

        a_count.abs_diff(b_count).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_length_after_removals(s: String) -> i32 {
        Self::min_length_after_removals(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
