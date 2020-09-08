pub struct Solution;

impl Solution {
    fn max_sum_from_one_side<I: IntoIterator<Item = i32>>(iter: I) -> i32 {
        let mut max_sum = i32::min_value();
        let mut sum = 0;

        for num in iter {
            sum += num;
            max_sum = max_sum.max(sum);
        }

        max_sum
    }

    pub fn max_sub_array_helper(nums: &[i32]) -> i32 {
        if nums.len() == 1 {
            nums[0]
        } else {
            let (left, right) = nums.split_at(nums.len() / 2);

            let left_sum = Self::max_sub_array_helper(left);
            let right_sum = Self::max_sub_array_helper(right);

            let middle_sum = Self::max_sum_from_one_side(left.iter().rev().copied())
                + Self::max_sum_from_one_side(right.iter().copied());

            left_sum.max(right_sum).max(middle_sum)
        }
    }

    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        Self::max_sub_array_helper(&nums)
    }
}

impl super::Solution for Solution {
    fn max_sub_array(nums: Vec<i32>) -> i32 {
        Self::max_sub_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
