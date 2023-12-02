pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU8;

impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut sequence = sequence.into_bytes();
        let word = word.into_bytes();
        let word_len = NonZeroU8::new(word.len() as _).unwrap();
        let mut max_matched = 0;
        let mut sequence_matched = 0_u8;
        let mut prefix_table_length = 0_u8;
        let mut word_matched = 0_u8;
        let mut i = 0;

        while let Some(&c) = sequence.get(i) {
            loop {
                if c == word[usize::from(sequence_matched % word_len)] {
                    sequence_matched += 1;

                    break;
                } else if sequence_matched == 0 {
                    break;
                }

                if sequence_matched > prefix_table_length {
                    // We need to compute more KMP prefix table items.

                    if prefix_table_length == 0 {
                        sequence[0] = 0;
                        prefix_table_length += 1;
                    }

                    for j in prefix_table_length..sequence_matched {
                        loop {
                            if word[usize::from(j % word_len)] == word[usize::from(word_matched % word_len)] {
                                word_matched += 1;

                                break;
                            } else if let Some(&next_word_matched) =
                                sequence.get(usize::from(word_matched).wrapping_sub(1))
                            {
                                word_matched = next_word_matched;
                            } else {
                                break;
                            }
                        }

                        sequence[usize::from(j)] = word_matched;
                    }

                    prefix_table_length = sequence_matched;
                }

                sequence_matched = sequence[usize::from(sequence_matched) - 1];
            }

            max_matched = max_matched.max(sequence_matched);

            i += 1;
        }

        i32::from(max_matched / word_len)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_repeating(sequence: String, word: String) -> i32 {
        Self::max_repeating(sequence, word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
