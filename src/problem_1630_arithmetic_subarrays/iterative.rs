pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;
use std::num::NonZeroU32;

impl Solution {
    fn min_max(nums: &[i32]) -> (i32, i32) {
        let mut iter = nums.iter().copied();
        let mut min = iter.next().unwrap();
        let mut max = min;

        while let Some(left) = iter.next() {
            if let Some(right) = iter.next() {
                let (new_min, new_max) = if right < left { (right, left) } else { (left, right) };

                min = min.min(new_min);
                max = max.max(new_max);
            } else {
                if left < min {
                    min = left;
                } else if left > max {
                    max = left;
                }

                break;
            }
        }

        (min, max)
    }

    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        let mut seen = Vec::new();

        l.iter()
            .zip(&r)
            .map(|(&start, &end)| {
                let window = &nums[start as u32 as usize..=end as u32 as usize];
                let (min, max) = Self::min_max(window);

                min == max || {
                    let range = (max - min) as u32;
                    let base = (end - start) as u32;

                    range % base == 0 && {
                        let interval = NonZeroU32::new(range / base).unwrap();

                        seen.resize(window.len(), false);

                        let result = window.iter().all(|&num| {
                            let num_minus_min = (num - min) as u32;

                            num_minus_min % interval == 0
                                && seen
                                    .get_mut((num_minus_min / interval) as usize)
                                    .map_or(false, |seen_value| !mem::replace(seen_value, true))
                        });

                        seen.clear();

                        result
                    }
                }
            })
            .collect()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        Self::check_arithmetic_subarrays(nums, l, r)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
