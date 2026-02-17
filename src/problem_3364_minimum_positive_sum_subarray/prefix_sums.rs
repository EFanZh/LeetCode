pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut nums = nums;
        let mut sum = 0;

        for num in &mut nums {
            sum += *num;
            *num = sum;
        }

        let mut result = u32::MAX;

        for window_size in l.cast_unsigned() as usize..=r.cast_unsigned() as usize {
            let mut left = 0;

            nums.iter()
                .zip(&nums[window_size - 1..])
                .for_each(|(&new_left, right)| {
                    let sum = right - left;

                    if sum > 0 {
                        result = result.min(sum.cast_unsigned());
                    }

                    left = new_left;
                });
        }

        result.cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        Self::minimum_sum_subarray(nums, l, r)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
