pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn max_palindromes_after_operations(words: Vec<String>) -> i32 {
        let mut counts = [0_u32; 26];

        let mut word_lengths = words
            .into_iter()
            .map(|word| {
                for c in word.bytes() {
                    counts[usize::from(c) - usize::from(b'a')] += 1;
                }

                word.len() as u16
            })
            .collect::<Vec<_>>();

        let mut even_pairs = 0;

        for count in counts {
            even_pairs += (count / 2) as u16;
        }

        word_lengths.sort_unstable();

        word_lengths
            .iter()
            .take_while(|&&length| {
                let required_even_pairs = length / 2;

                if even_pairs < required_even_pairs {
                    false
                } else {
                    even_pairs -= required_even_pairs;

                    true
                }
            })
            .count() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_palindromes_after_operations(words: Vec<String>) -> i32 {
        Self::max_palindromes_after_operations(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
