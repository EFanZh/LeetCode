pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    fn get_unique_count(nums: &[u32]) -> u16 {
        let mut result = 0;
        let mut seen = [false; 2000];

        for &num in nums {
            result += u16::from(!mem::replace(&mut seen[num as usize], true));
        }

        result
    }

    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(|x| x as u32 - 1).collect::<Vec<_>>();
        let total_unique_count = Self::get_unique_count(&nums);
        let mut result = 0;
        let mut counts = [0_u16; 2000];
        let mut start = 0;
        let mut window_unique_count = 0;

        for &num in &nums {
            let num = num as usize;
            let count = &mut counts[num];

            window_unique_count += u16::from(*count == 0);
            *count += 1;

            while window_unique_count == total_unique_count {
                let start_count = &mut counts[nums[start] as usize];

                *start_count -= 1;
                window_unique_count -= u16::from(*start_count == 0);
                start += 1;
            }

            result += start as i32;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        Self::count_complete_subarrays(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
