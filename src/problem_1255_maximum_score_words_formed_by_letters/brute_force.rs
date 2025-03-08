pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::slice::Iter;

impl Solution {
    fn helper(scores: &[u8; 26], score: u32, mut words: Iter<String>, letters: &mut [u8; 26], result: &mut u32) {
        if let Some(word) = words.next().map(String::as_str) {
            let mut counts = [0; 26];

            for c in word.bytes() {
                counts[usize::from(c) - usize::from(b'a')] += 1;
            }

            if letters.iter().zip(counts).all(|(lhs, rhs)| *lhs >= rhs) {
                for (lhs, rhs) in letters.iter_mut().zip(counts) {
                    *lhs -= rhs;
                }

                let new_score = score
                    + counts
                        .iter()
                        .zip(scores)
                        .map(|(&count, &score)| u32::from(count) * u32::from(score))
                        .sum::<u32>();

                Self::helper(scores, new_score, words.clone(), letters, result);

                for (lhs, rhs) in letters.iter_mut().zip(counts) {
                    *lhs += rhs;
                }
            }

            Self::helper(scores, score, words, letters, result);
        } else {
            *result = (*result).max(score);
        }
    }

    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let mut new_letters = [0; 26];

        for c in letters {
            new_letters[c as usize - usize::from(b'a')] += 1;
        }

        let scores = <[_; 26]>::map(score.try_into().ok().unwrap(), |score| score as u8);
        let mut result = 0;

        Self::helper(&scores, 0, words.iter(), &mut new_letters, &mut result);

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        Self::max_score_words(words, letters, score)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
