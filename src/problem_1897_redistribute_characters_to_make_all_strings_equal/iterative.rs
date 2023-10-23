pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU16;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let n = NonZeroU16::new(words.len() as _).unwrap();
        let mut counts = [0; 26];

        for word in words {
            for c in word.bytes() {
                counts[usize::from(c) - usize::from(b'a')] += 1;
            }
        }

        counts.iter().all(|&count| count % n == 0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn make_equal(words: Vec<String>) -> bool {
        Self::make_equal(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
