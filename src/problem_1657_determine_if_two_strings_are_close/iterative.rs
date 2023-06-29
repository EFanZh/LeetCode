pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn check_existing_letters(s: &[u32; 26]) -> u32 {
        let mut result = 0;

        for (i, &c) in s.iter().enumerate() {
            result |= u32::from(c != 0) << i;
        }

        result
    }

    pub fn close_strings(word1: String, word2: String) -> bool {
        word1.len() == word2.len() && {
            let mut counts_1 = [0_u32; 26];
            let mut counts_2 = [0_u32; 26];

            for (counts, word) in [(&mut counts_1, word1.bytes()), (&mut counts_2, word2.bytes())] {
                for c in word {
                    counts[usize::from(c) - usize::from(b'a')] += 1;
                }
            }

            Self::check_existing_letters(&counts_1) == Self::check_existing_letters(&counts_2) && {
                counts_1.sort_unstable();
                counts_2.sort_unstable();

                counts_1 == counts_2
            }
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn close_strings(word1: String, word2: String) -> bool {
        Self::close_strings(word1, word2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
