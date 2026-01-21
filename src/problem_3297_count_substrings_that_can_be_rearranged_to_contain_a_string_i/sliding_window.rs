pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn valid_substring_count(word1: String, word2: String) -> i64 {
        let word1 = word1.into_bytes();
        let word2 = word2.into_bytes();
        let mut counts = [0_i32; 26];

        for c in word2 {
            counts[usize::from(c) - usize::from(b'a')] -= 1;
        }

        let mut start = 0;
        let mut miss = counts.iter().map(|&count| u8::from(count != 0)).sum::<u8>();
        let mut result = 0;

        for &c in &word1 {
            let count = &mut counts[usize::from(c) - usize::from(b'a')];

            *count += 1;
            miss -= u8::from(*count == 0);

            while miss == 0 {
                let count = &mut counts[usize::from(word1[start]) - usize::from(b'a')];

                miss += u8::from(*count == 0);
                *count -= 1;
                start += 1;
            }

            result += start;
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn valid_substring_count(word1: String, word2: String) -> i64 {
        Self::valid_substring_count(word1, word2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
