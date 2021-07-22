pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::{Ordering, Reverse};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::Hasher;
use std::mem;

impl Solution {
    fn select_pivot(length: usize, rng: &mut impl Hasher) -> usize {
        rng.write_usize(length);

        (rng.finish() % (length as u64)) as _
    }

    fn partition<T: Ord>(values: &mut [T], rng: &mut impl Hasher) -> usize {
        let length = values.len();
        let pivot = Self::select_pivot(length, rng);

        values.swap(pivot, length - 1);

        let (key, rest) = values.split_last_mut().unwrap();
        let mut left = 0;
        let mut right = rest.len();

        'outer: while left != right {
            if rest[left] < *key {
                left += 1;
            } else {
                loop {
                    right -= 1;

                    if left == right {
                        break 'outer;
                    }

                    if rest[right] < *key {
                        rest.swap(left, right);
                        left += 1;

                        break;
                    }
                }
            }
        }

        values.swap(left, length - 1);

        left
    }

    fn select_nth<T: Ord>(mut values: &mut [T], mut n: usize) {
        let mut rng = DefaultHasher::new();

        while values.len() > 1 {
            let pivot = Self::partition(values, &mut rng);

            match pivot.cmp(&n) {
                Ordering::Less => {
                    values = &mut values[pivot + 1..];
                    n -= pivot + 1;
                }
                Ordering::Equal => break,
                Ordering::Greater => values = &mut values[..pivot],
            }
        }
    }

    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let k = k as usize;
        let mut frequencies = HashMap::new();

        for word in words {
            frequencies.entry(word).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut frequencies = frequencies
            .into_iter()
            .map(|(word, frequency)| (Reverse(frequency), word))
            .collect::<Vec<_>>();

        Self::select_nth(&mut frequencies, k);

        let results = &mut frequencies[..k];

        results.sort_unstable();

        results.iter_mut().map(|(_, word)| mem::take(word)).collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        Self::top_k_frequent(words, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
