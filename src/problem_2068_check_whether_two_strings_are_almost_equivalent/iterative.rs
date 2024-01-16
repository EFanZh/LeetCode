pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let mut counts = [0_i8; 26];

        for c in word1.into_bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        for c in word2.into_bytes() {
            counts[usize::from(c) - usize::from(b'a')] -= 1;
        }

        counts.iter().all(|&count| matches!(count, -3..=3))
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_almost_equivalent(word1: String, word2: String) -> bool {
        Self::check_almost_equivalent(word1, word2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
