pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::num::NonZeroU32;

impl Solution {
    fn gcd(mut x: u32, mut y: NonZeroU32) -> NonZeroU32 {
        while let Some(z) = NonZeroU32::new(x % y) {
            x = y.get();
            y = z;
        }

        y
    }

    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
        let k = NonZeroU32::new(k as _).unwrap();
        let mut counts = HashMap::<_, u64>::new();

        for num in nums {
            match counts.entry(Self::gcd(num as _, k)) {
                Entry::Occupied(entry) => *entry.into_mut() += 1,
                Entry::Vacant(entry) => {
                    entry.insert(1);
                }
            }
        }

        let mut result = 0;

        for (&gcd_1, &count_1) in &counts {
            let required = NonZeroU32::new(k.get() / gcd_1).unwrap();

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

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_pairs(nums: Vec<i32>, k: i32) -> i64 {
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
