pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let s = s.into_bytes();
        let k = k as usize;
        let n = s.len();
        let max_group_length = n + 1 - k;
        let mut changes = vec![0_u8; n * (max_group_length - 1)]; // Changes required to make a substring palindrome.
        let changes_index = |length: usize, start: usize| n.wrapping_mul(length.wrapping_sub(2)) + start;

        for length in 2..=max_group_length {
            for start in 0..=n - length {
                changes[changes_index(length, start)] = u8::from(s[start] != s[start + length - 1])
                    + changes.get(changes_index(length - 2, start + 1)).copied().unwrap_or(0);
            }
        }

        let mut cache = vec![0; n * 2];
        let (mut cache, mut temp) = cache.split_at_mut(n);

        for (target, &value) in cache[1..].iter_mut().zip(changes.iter().step_by(n)) {
            *target = value;
        }

        for k in 2..=k {
            for (length, value) in (k..=n).zip(&mut *temp) {
                *value = (k - 1..)
                    .zip(&cache[..=length - k])
                    .map(|(split, &value)| {
                        value + changes.get(changes_index(length - split, split)).copied().unwrap_or(0)
                    })
                    .min()
                    .unwrap();
            }

            mem::swap(&mut cache, &mut temp);
        }

        i32::from(cache[n - k])
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn palindrome_partition(s: String, k: i32) -> i32 {
        Self::palindrome_partition(s, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
