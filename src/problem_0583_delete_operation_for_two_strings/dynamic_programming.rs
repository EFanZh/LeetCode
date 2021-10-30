pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let (word1, word2) = if word2.len() < word1.len() {
            (word2, word1)
        } else {
            (word1, word2)
        };

        let (&word1_first, word1_rest) = word1.as_bytes().split_first().unwrap();
        let mut cache_first = 1;
        let mut cache_rest = (0..word1_rest.len()).map(|x| x + 2).collect::<Vec<_>>();

        for (i, c2) in word2.bytes().enumerate() {
            let mut left_top = cache_first;

            cache_first = if word1_first == c2 { i } else { cache_first + 1 };

            let mut left = cache_first;

            for (slot, &c1) in cache_rest.iter_mut().zip(word1_rest) {
                left_top = mem::replace(slot, if c1 == c2 { left_top } else { (*slot).min(left) + 1 });
                left = *slot;
            }
        }

        cache_rest.last().copied().unwrap_or(cache_first) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
