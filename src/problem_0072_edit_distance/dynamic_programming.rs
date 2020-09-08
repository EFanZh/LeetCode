pub struct Solution;

use std::mem;

impl Solution {
    pub fn min_distance(mut word1: String, mut word2: String) -> i32 {
        if word2.len() < word1.len() {
            mem::swap(&mut word1, &mut word2);
        }

        let mut cache = (0..=word1.len() as _).rev().collect::<Box<_>>();

        for (prev_base, c2) in word2.as_bytes().iter().rev().enumerate() {
            let mut prev = prev_base as _;

            cache[word1.len()] = prev + 1;

            for (i, c1) in word1.as_bytes().iter().enumerate().rev() {
                let distance = if c1 == c2 {
                    prev
                } else {
                    cache[i].min(cache[i + 1]).min(prev) + 1
                };

                prev = mem::replace(&mut cache[i], distance);
            }
        }

        cache[0]
    }
}

impl super::Solution for Solution {
    fn min_distance(word1: String, word2: String) -> i32 {
        Self::min_distance(word1, word2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
