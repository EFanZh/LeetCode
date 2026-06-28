pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        let weights = <[_; 26]>::try_from(&*weights).unwrap();

        String::from_utf8(
            words
                .into_iter()
                .map(|word| {
                    b'a' + 25
                        - (word
                            .bytes()
                            .map(|c| weights[usize::from(c) - usize::from(b'a')])
                            .sum::<i32>()
                            .cast_unsigned()
                            % 26) as u8
                })
                .collect::<Vec<u8>>(),
        )
        .unwrap()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        Self::map_word_weights(words, weights)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
