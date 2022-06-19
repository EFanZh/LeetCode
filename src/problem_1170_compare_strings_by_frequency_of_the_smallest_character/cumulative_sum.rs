pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;

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
        let mut frequencies = [0; 11];

        for word in &words {
            let value = Self::f(word);

            frequencies[(usize::from(value) - 1).min(10)] += 1;
        }

        drop(words);

        let mut prev = 0;

        for frequency in frequencies.iter_mut().rev() {
            *frequency += prev;
            prev = *frequency;
        }

        queries.iter().map(|s| frequencies[usize::from(Self::f(s))]).collect()
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
