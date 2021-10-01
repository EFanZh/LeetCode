pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;

impl Solution {
    fn select_pivot(length: usize, rng: &mut impl Hasher) -> usize {
        rng.write_usize(length);

        (rng.finish() % (length as u64)) as _
    }

    fn partition(values: &mut [i32], rng: &mut impl Hasher) -> usize {
        let length = values.len();
        let pivot = Self::select_pivot(length, rng);

        values.swap(pivot, length - 1);

        let mut left = 0;
        let mut right = length - 1;
        let (&mut key, rest) = values.split_last_mut().unwrap();

        'outer: while left != right {
            if rest[left] < key {
                left += 1;
            } else {
                loop {
                    right -= 1;

                    if left == right {
                        break 'outer;
                    }

                    if rest[right] < key {
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

    fn select_nth(mut values: &mut [i32], mut n: usize) {
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

    pub fn min_moves2(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            0
        } else {
            let half = nums.len() / 2;

            Self::select_nth(&mut nums, half);

            let median = nums[half];

            nums.iter().map(|num| (num - median).abs()).sum()
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_moves2(nums: Vec<i32>) -> i32 {
        Self::min_moves2(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
