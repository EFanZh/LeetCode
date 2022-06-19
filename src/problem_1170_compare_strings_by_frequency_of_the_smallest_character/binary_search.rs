pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::{Ordering, Reverse};

impl Solution {
    fn f(s: &str) -> u16 {
        let mut min_char = u8::MAX;
        let mut count = 0;

        for c in s.bytes() {
            match c.cmp(&min_char) {
                Ordering::Less => {
                    min_char = c;
                    count = 1;
                }
                Ordering::Equal => count += 1,
                Ordering::Greater => {}
            }
        }

        count
    }

    pub fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        let mut frequencies = words.iter().map(|s| Self::f(s)).collect::<Vec<_>>();

        drop(words);

        frequencies.sort_unstable_by_key(|&value| Reverse(value));

        queries
            .iter()
            .map(|s| frequencies.partition_point(|&value| value > Self::f(s)) as _)
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn num_smaller_by_frequency(queries: Vec<String>, words: Vec<String>) -> Vec<i32> {
        Self::num_smaller_by_frequency(queries, words)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
