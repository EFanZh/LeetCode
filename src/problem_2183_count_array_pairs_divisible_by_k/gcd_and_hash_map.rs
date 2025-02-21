pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::collections::HashMap;
use std::collections::hash_map::Entry;
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
        let mut factors = Vec::new();
        let middle = NonZeroU32::new(f64::from(k.get()).sqrt() as _).unwrap();

        for candidate in 1..middle.get() {
            let candidate = NonZeroU32::new(candidate).unwrap();

            if k.get() % candidate == 0 {
                factors.extend([candidate, NonZeroU32::new(k.get() / candidate).unwrap()]);
            }
        }

        if k.get() % middle == 0 {
            if middle.get() * middle.get() == k.get() {
                factors.push(middle);
            } else {
                factors.extend([middle, NonZeroU32::new(k.get() / middle).unwrap()]);
            }
        }

        let mut counts = HashMap::<_, u32>::new();

        nums.into_iter().fold(0_u64, |mut result, num| {
            let num = num as u32;

            result += u64::from(counts.get(&(k.get() / Self::gcd(num, k))).copied().unwrap_or(0));

            for &factor in &factors {
                if num % factor == 0 {
                    match counts.entry(factor.get()) {
                        Entry::Occupied(entry) => *entry.into_mut() += 1,
                        Entry::Vacant(entry) => {
                            entry.insert(1);
                        }
                    }
                }
            }

            result
        }) as _
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
