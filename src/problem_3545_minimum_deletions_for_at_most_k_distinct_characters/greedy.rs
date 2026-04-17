pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_deletion(s: String, k: i32) -> i32 {
        let mut counts = [0_u8; 26];

        for c in s.bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        counts
            .select_nth_unstable(26 - k.cast_unsigned() as usize)
            .0
            .iter()
            .fold(0, |sum, &count| sum + i32::from(count))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_deletion(s: String, k: i32) -> i32 {
        Self::min_deletion(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
