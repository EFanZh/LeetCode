pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    /// Get the number of subarrays whose sum is less than or equal to `max_sum`.
    fn count_subarrays(sums: &[i32], max_sum: u32) -> u32 {
        let mut start = usize::MAX;
        let mut left = 0;
        let mut result = 0;

        for (end, &right) in sums.iter().enumerate() {
            let right = right as u32;

            while right - left > max_sum {
                start = start.wrapping_add(1);
                left = sums[start] as _;
            }

            result += end.wrapping_sub(start) as u32;
        }

        result
    }

    /// Get the sum of sums of subarrays whose sum is less than or equal to `max_sum`.
    fn sum_subarray_sums(sums: &[i32], max_sum: u32) -> u64 {
        let mut start = usize::MAX;
        let mut left = 0;
        let mut sum = 0;
        let mut result = 0;

        for (end, &right) in sums.iter().enumerate() {
            let right = right as u32;

            while right - left > max_sum {
                sum -= left;
                start = start.wrapping_add(1);
                left = sums[start] as _;
            }

            result += u64::from(right) * end.wrapping_sub(start) as u64 - u64::from(sum);
            sum += right;
        }

        result
    }

    /// Get the `i`-th subarray sum.
    fn get_subarray_sum(sums: &[i32], mut low: u32, mut high: u32, i: u32) -> u32 {
        while low < high {
            let middle = (low + high) / 2;

            if Self::count_subarrays(sums, middle) <= i {
                low = middle + 1;
            } else {
                high = middle;
            }
        }

        low
    }

    /// Get the `n`-th subarray sum and the sum of the smallest `n` subarray sums.
    fn get_subarray_prefix_sum(sums: &[i32], low: u32, high: u32, n: u32) -> (u32, u64) {
        let nth_subarray_sum = Self::get_subarray_sum(sums, low, high, n);
        let sum_of_subarray_sums = Self::sum_subarray_sums(sums, nth_subarray_sum);
        let total_subarrays = Self::count_subarrays(sums, nth_subarray_sum);
        let prefix_sum = sum_of_subarray_sums - u64::from(nth_subarray_sum) * u64::from(total_subarrays - n);

        (nth_subarray_sum, prefix_sum)
    }

    pub fn range_sum(nums: Vec<i32>, _n: i32, left: i32, right: i32) -> i32 {
        let mut nums = nums;
        let mut min_num = nums[0] as u32;
        let mut sum = 0;

        for value in &mut nums {
            min_num = min_num.min(*value as _);
            sum += *value;
            *value = sum;
        }

        let nums = nums.as_slice();
        let sum = sum as u32;
        let (new_min_sum, left) = Self::get_subarray_prefix_sum(nums, min_num, sum, left as u32 - 1);
        let right = Self::get_subarray_prefix_sum(nums, new_min_sum, sum, right as _).1;

        ((right - left) % 1_000_000_007) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        Self::range_sum(nums, n, left, right)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
