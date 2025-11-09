pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::num::NonZero;

impl Solution {
    fn gcd(mut x: u32, mut y: u32) -> u32 {
        while y != 0 {
            (x, y) = (y, x % y);
        }

        x
    }

    fn get_factors(value: NonZero<u32>) -> Vec<NonZero<u32>> {
        let sqrt_value = value.isqrt();

        let mut result = (2..sqrt_value.get())
            .filter_map(NonZero::new)
            .filter(|&x| value.get() % x == 0)
            .collect::<Vec<_>>();

        let n = result.len();

        if sqrt_value.get() * sqrt_value.get() == value.get() {
            result.push(sqrt_value);
        }

        result.reserve(n + 1);

        for i in (0..n).rev() {
            result.extend(result.get(i).copied().and_then(|x| NonZero::new(value.get() / x)));
        }

        result.push(value);

        result
    }

    pub fn min_anagram_length(s: String) -> i32 {
        let mut counts = HashMap::<_, u32>::new();

        s.bytes().for_each(|chunk| match counts.entry(chunk) {
            Entry::Occupied(occupied_entry) => *occupied_entry.into_mut() += 1,
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(1);
            }
        });

        let gcd = counts.values().copied().fold(0, Self::gcd);

        let mut counts = [0_u32; 26];

        let prefix_sums = s
            .bytes()
            .map(|c| {
                counts[usize::from(c) - usize::from(b'a')] += 1;

                counts
            })
            .collect::<Box<_>>();

        let gcd = NonZero::new(gcd).unwrap();
        let gcd_factors = Self::get_factors(gcd);

        for &repeats in gcd_factors.iter().rev() {
            let chunk_size = s.len() / repeats.get() as usize;

            if (chunk_size - 1..)
                .step_by(chunk_size)
                .map_while(|i| {
                    prefix_sums.get(i).map(|right| {
                        let left = prefix_sums
                            .get(i.wrapping_sub(chunk_size))
                            .unwrap_or(const { &[0; 26] });

                        (left, right)
                    })
                })
                .all(|(left, right)| {
                    right
                        .iter()
                        .zip(left)
                        .zip(&counts)
                        .all(|((right, left), &total)| (right - left) * repeats.get() == total)
                })
            {
                return chunk_size as _;
            }
        }

        s.len() as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_anagram_length(s: String) -> i32 {
        Self::min_anagram_length(s)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
