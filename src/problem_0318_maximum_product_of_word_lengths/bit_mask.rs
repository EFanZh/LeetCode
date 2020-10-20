pub struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut word_ids = vec![0; words.len()];

        for (target, word) in word_ids.iter_mut().zip(&words) {
            for c in word.bytes() {
                *target |= 1 << u32::from(c - b'a');
            }
        }

        let mut result = 0;

        for (i, (lhs, lhs_id)) in words.iter().zip(&word_ids).enumerate().take(words.len() - 1) {
            for (rhs, rhs_id) in words.iter().zip(&word_ids).skip(i + 1) {
                if lhs_id & rhs_id == 0 {
                    result = result.max(lhs.len() * rhs.len());
                }
            }
        }

        result as _
    }
}

impl super::Solution for Solution {
    fn max_product(words: Vec<String>) -> i32 {
        Self::max_product(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
