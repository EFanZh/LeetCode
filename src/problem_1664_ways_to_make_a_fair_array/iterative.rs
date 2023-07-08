pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

use std::mem;

impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let buffer = nums.as_mut_slice();
        let mut sum_1 = 0;
        let mut sum_2 = 0;

        for target in &mut *buffer {
            sum_1 += *target;
            *target = sum_1;

            mem::swap(&mut sum_1, &mut sum_2);
        }

        if buffer.len() % 2 != 0 {
            mem::swap(&mut sum_1, &mut sum_2);
        }

        let mut result = 0;
        let mut prev_1 = 0;
        let mut prev_2 = 0;

        for &sum in &*buffer {
            result += i32::from((prev_1 + (sum_2 - prev_2)) == (prev_2 + (sum_1 - sum)));

            prev_1 = prev_2;
            prev_2 = sum;
            mem::swap(&mut sum_1, &mut sum_2);
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        Self::ways_to_make_fair(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
