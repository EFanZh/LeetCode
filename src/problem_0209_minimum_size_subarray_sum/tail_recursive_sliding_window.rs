pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper_less(s: i32, nums: &[i32], start: usize, mut end: usize, mut sum: i32, result: usize) -> usize {
        nums.get(end).map_or(result, |num| {
            end += 1;
            sum += num;

            if sum < s {
                Self::helper_less(s, nums, start, end, sum, result)
            } else {
                Self::helper_not_less(s, nums, start, end, sum, result)
            }
        })
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
        let result = Self::helper_less(s, &nums, 0, 0, 0, usize::MAX);

        if result == usize::MAX {
            0
        } else {
            result as _
        }
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

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
