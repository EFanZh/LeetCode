pub struct Solution;

impl Solution {
    fn helper_less(s: i32, nums: &[i32], start: usize, mut end: usize, mut sum: i32, result: usize) -> usize {
        if let Some(num) = nums.get(end) {
            end += 1;
            sum += num;

            if sum < s {
                Self::helper_less(s, nums, start, end, sum, result)
            } else {
                Self::helper_not_less(s, nums, start, end, sum, result)
            }
        } else {
            result
        }
    }

    fn helper_not_less(s: i32, nums: &[i32], start: usize, end: usize, mut sum: i32, result: usize) -> usize {
        sum -= nums[start];

        if sum < s {
            Self::helper_less(s, nums, start + 1, end, sum, result.min(end - start))
        } else {
            Self::helper_not_less(s, nums, start + 1, end, sum, result)
        }
    }

    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        let result = Self::helper_less(s, &nums, 0, 0, 0, usize::max_value());

        if result == usize::max_value() {
            0
        } else {
            result as _
        }
    }
}

impl super::Solution for Solution {
    fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        Self::min_sub_array_len(s, nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
