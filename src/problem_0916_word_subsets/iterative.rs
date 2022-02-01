pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn count_letters(word: &str) -> [u8; 26] {
        let mut counts = [0; 26];

        for c in word.bytes() {
            counts[usize::from(c) - usize::from(b'a')] += 1;
        }

        counts
    }

    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let mut required = [0_u8; 26];

        for word in &words2 {
            for (target, &source) in required.iter_mut().zip(&Self::count_letters(word)) {
                *target = (*target).max(source);
            }
        }

        let mut words1 = words1;

        words1.retain(|word| Self::count_letters(word).iter().zip(&required).all(|(c, r)| c >= r));

        words1
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        Self::word_subsets(words1, words2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
