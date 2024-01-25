pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::num::NonZeroU64;

impl Solution {
    pub fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as u32 as usize;
        let window_size = k * 2 + 1;
        let window_size_u64 = NonZeroU64::new(window_size as _).unwrap();

        if let Some(right) = nums.get(window_size..) {
            let mut result = vec![-1; nums.len()];

            let mut sum = nums[..window_size]
                .iter()
                .fold(0_u64, |sum, &x| sum + u64::from(x as u32));

            result[k] = (sum / window_size_u64) as _;

            for (target, (old, new)) in result[k + 1..].iter_mut().zip(nums.iter().zip(right)) {
                sum = sum.wrapping_add((new - old) as _);

                *target = (sum / window_size_u64) as _;
            }

            result
        } else {
            let mut nums = nums;

            nums.fill(-1);

            nums
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn get_averages(nums: Vec<i32>, k: i32) -> Vec<i32> {
        Self::get_averages(nums, k)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
