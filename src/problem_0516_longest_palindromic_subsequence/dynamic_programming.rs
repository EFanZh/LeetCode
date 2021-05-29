pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::iter;
use std::mem;

impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let s = s.as_bytes();

        let mut cache = iter::repeat(0)
            .take(s.len())
            .chain(iter::repeat(1).take(s.len()))
            .collect::<Box<_>>();

        let (mut cache_1, mut cache_2) = cache.split_at_mut(s.len());

        for length in 2..=s.len() {
            for i in 0..=s.len() - length {
                cache_1[i] = if s[i] == s[i + (length - 1)] {
                    cache_1[i + 1] + 2
                } else {
                    cache_2[i].max(cache_2[i + 1])
                };
            }

            mem::swap(&mut cache_1, &mut cache_2);
        }

        cache_2.first().copied().unwrap_or(0)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_palindrome_subseq(s: String) -> i32 {
        Self::longest_palindrome_subseq(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
