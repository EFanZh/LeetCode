pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut counts = [u8::MAX; 26];

        for word in &words {
            let mut buffer = [u8::MIN; 26];

            for c in word.bytes() {
                buffer[usize::from(c) - usize::from(b'a')] += 1;
            }

            for (target, &count) in counts.iter_mut().zip(&buffer) {
                *target = (*target).min(count);
            }
        }

        let mut result = Vec::with_capacity(usize::from(counts.iter().sum::<u8>()));

        for (c, count) in (b'a'..).zip(counts) {
            for _ in 0..count {
                result.push(char::from(c).to_string());
            }
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn common_chars(words: Vec<String>) -> Vec<String> {
        Self::common_chars(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
