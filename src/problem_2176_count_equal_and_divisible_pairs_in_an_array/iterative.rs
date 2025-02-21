pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::num::NonZero;

impl Solution {
    fn gcd(mut x: u8, mut y: NonZero<u8>) -> NonZero<u8> {
        while let Some(z) = NonZero::<u8>::new(x % y) {
            x = y.get();
            y = z;
        }

        y
    }

    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let k = NonZero::<u8>::new(k as _).unwrap();
        let mut indices = [const { Vec::new() }; 100];

        for (i, num) in (0_u8..).zip(nums) {
            indices[num as u32 as usize - 1].push(i);
        }

        // See solutions to <https://leetcode.com/problems/count-array-pairs-divisible-by-k/>.

        let mut counts = HashMap::<_, u16>::new();
        let mut result = 0;

        for indices in indices {
            for index in indices {
                match counts.entry(Self::gcd(index, k)) {
                    Entry::Occupied(entry) => *entry.into_mut() += 1,
                    Entry::Vacant(entry) => {
                        entry.insert(1);
                    }
                }
            }

            for (&gcd_1, &count_1) in &counts {
                let required = NonZero::<u8>::new(k.get() / gcd_1).unwrap();

                for (&gcd_2, &count_2) in &counts {
                    if gcd_2.get() % required == 0 {
                        result += match gcd_1.cmp(&gcd_2) {
                            Ordering::Less => count_1 * count_2,
                            Ordering::Equal => count_1 * (count_1 - 1) / 2,
                            Ordering::Greater => continue,
                        };
                    }
                }
            }

            counts.clear();
        }

        u32::from(result) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        Self::count_pairs(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
