pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut result = 0;

        if words.len() > 1 {
            let mut word_ids = vec![0; words.len()];

            for (target, word) in word_ids.iter_mut().zip(&words) {
                for c in word.bytes() {
                    *target |= 1 << u32::from(c - b'a');
                }
            }

            for (i, lhs_id) in word_ids[..word_ids.len() - 1].iter().enumerate() {
                for (j, rhs_id) in word_ids.iter().enumerate().skip(i + 1) {
                    if lhs_id & rhs_id == 0 {
                        result = result.max(words[i].len() * words[j].len());
                    }
                }
            }
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
