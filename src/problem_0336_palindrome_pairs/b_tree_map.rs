pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    fn is_palindrome(s: &[u8]) -> bool {
        let mut iter = s.iter();

        while let Some(lhs) = iter.next() {
            if let Some(rhs) = iter.next_back() {
                if lhs != rhs {
                    return false;
                }
            } else {
                break;
            }
        }

        true
    }

    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let word_map = words.iter().map(String::as_bytes).zip(0..).collect::<BTreeMap<_, _>>();
        let mut result = Vec::new();

        for (i, reversed_word) in (0..).zip(words.iter().map(|word| word.bytes().rev().collect::<Box<_>>())) {
            if let Some(&j) = word_map.get(reversed_word.as_ref()) {
                if j > i {
                    result.push(vec![i, j]);
                    result.push(vec![j, i]);
                }
            }

            for length in 0..reversed_word.len() {
                let (left, right) = reversed_word.split_at(reversed_word.len() - length);

                if Self::is_palindrome(left) {
                    if let Some(&j) = word_map.get(right) {
                        result.push(vec![i, j]);
                    }
                }

                let (left, right) = reversed_word.split_at(length);

                if Self::is_palindrome(right) {
                    if let Some(&j) = word_map.get(left) {
                        result.push(vec![j, i]);
                    }
                }
            }
        }

        result
    }
}

impl super::Solution for Solution {
    fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        Self::palindrome_pairs(words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
