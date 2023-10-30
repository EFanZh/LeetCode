pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut counts = [0_u32; 1024];
        let mut state = 0;
        let mut result = 0;

        counts[0] = 1;

        for c in word.into_bytes() {
            state ^= 1 << (c - b'a');

            result += i64::from(counts[state]);

            let mut probe = 1 << 9;

            loop {
                result += i64::from(counts[state ^ probe]);

                probe >>= 1;

                if probe == 0 {
                    break;
                }
            }

            counts[state] += 1;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn wonderful_substrings(word: String) -> i64 {
        Self::wonderful_substrings(word)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
