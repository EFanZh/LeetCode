pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut word1 = word1.as_bytes();
        let mut word2 = word2.as_bytes();

        let rest = if word2.len() < word1.len() {
            let (new_word1, rest) = word1.split_at(word2.len());

            word1 = new_word1;

            rest
        } else {
            let (new_word2, rest) = word2.split_at(word1.len());

            word2 = new_word2;

            rest
        };

        let mut result = Vec::with_capacity(word1.len() + word2.len() + rest.len());

        for (&left, &right) in word1.iter().zip(word2) {
            result.push(left);
            result.push(right);
        }

        result.extend(rest);

        String::from_utf8(result).unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn merge_alternately(word1: String, word2: String) -> String {
        Self::merge_alternately(word1, word2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
