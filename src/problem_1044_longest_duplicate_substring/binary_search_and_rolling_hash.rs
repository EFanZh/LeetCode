pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Default)]
struct Pool {
    inner: Vec<Vec<usize>>,
}

impl Pool {
    fn allocate(&mut self, item: usize) -> Vec<usize> {
        self.inner.pop().map_or_else(
            || vec![item],
            |mut result| {
                result.push(item);

                result
            },
        )
    }

    fn free(&mut self, mut value: Vec<usize>) {
        value.clear();

        self.inner.push(value);
    }
}

struct PoolGuard<'a> {
    buckets: &'a mut HashMap<u64, Vec<usize>>,
    pool: &'a mut Pool,
}

impl PoolGuard<'_> {
    fn check(&mut self, s: &[u8], length: usize, i: usize, hash: u64) -> bool {
        match self.buckets.entry(hash) {
            Entry::Occupied(entry) => {
                if entry
                    .get()
                    .iter()
                    .any(|&start| s[start..start + length] == s[i..i + length])
                {
                    return true;
                }

                entry.into_mut().push(i);
            }
            Entry::Vacant(entry) => {
                entry.insert(self.pool.allocate(i));
            }
        }

        false
    }
}

impl Drop for PoolGuard<'_> {
    fn drop(&mut self) {
        for (_, bucket) in self.buckets.drain() {
            self.pool.free(bucket);
        }
    }
}

impl Solution {
    fn exp_mod(base: u64, exponent: usize, modulus: u64) -> u64 {
        // This algorithm has linear time complexity to avoid overflowing.

        let mut result = 1;

        for _ in 0..exponent {
            result = (result * base) % modulus;
        }

        result
    }

    fn helper<const M: u64>(
        s: &[u8],
        length: usize,
        buckets: &mut HashMap<u64, Vec<usize>>,
        pool: &mut Pool,
    ) -> Option<usize> {
        const D: u64 = 26;

        let mut pool_guard = PoolGuard { buckets, pool };
        let h = Self::exp_mod(D, length - 1, M);
        let (left, right) = s.split_at(length);
        let mut hash = 0;

        for &c in left {
            hash = (hash * D + u64::from(c) - u64::from(b'a')) % M;
        }

        pool_guard.buckets.insert(hash, pool_guard.pool.allocate(0));

        for (i, (&old, &new)) in (1..).zip(s.iter().zip(right)) {
            let old = u64::from(old) - u64::from(b'a');
            let new = u64::from(new) - u64::from(b'a');

            hash = (D * (hash + (M - (h * old) % M)) + new) % M;

            if pool_guard.check(s, length, i, hash) {
                return Some(i);
            }
        }

        None
    }

    pub fn longest_dup_substring(s: String) -> String {
        let mut left = 1;
        let mut right = s.len();
        let mut buckets = HashMap::new();
        let mut pool = Pool::default();
        let mut start = 0;
        let mut length = 0;

        while left < right {
            let middle = (left + right) / 2;

            if let Some(i) = Self::helper::<91_320_515_216_383_913>(s.as_bytes(), middle, &mut buckets, &mut pool) {
                start = i;
                length = middle;

                left = middle + 1;
            } else {
                right = middle;
            }
        }

        let mut s = s;

        s.truncate(start + length);
        s.replace_range(..start, "");

        s
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn longest_dup_substring(s: String) -> String {
        Self::longest_dup_substring(s)
    }
}

#[cfg(test)]
mod tests {
    use super::Pool;
    use std::collections::HashMap;

    #[test]
    fn test_helper() {
        let test_cases = [("aaa", Some(1)), ("abcd", None)];

        for (s, expected) in test_cases {
            assert_eq!(
                super::Solution::helper::<2>(s.as_bytes(), 2, &mut HashMap::new(), &mut Pool::default()),
                expected
            );
        }
    }

    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
