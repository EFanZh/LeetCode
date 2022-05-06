pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    fn helper(nums: &[i32], left_len: usize, right_len: usize, buffer: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        let mut right_sum = nums[n - right_len..].iter().copied().sum::<i32>();
        let mut max_right_sum = right_sum;

        for (&new, &old) in nums[left_len..].iter().zip(&nums[left_len + right_len..]).rev() {
            buffer.push(max_right_sum);

            right_sum = right_sum - old + new;
            max_right_sum = max_right_sum.max(right_sum);
        }

        let mut left_sum = nums[..left_len].iter().copied().sum::<i32>();
        let mut max_left_sum = left_sum;
        let mut result = max_left_sum + max_right_sum;

        for ((&old, &new), &max_right_sum) in nums[..n - (left_len + right_len)]
            .iter()
            .zip(&nums[left_len..])
            .zip(buffer.iter().rev())
        {
            left_sum = left_sum - old + new;
            max_left_sum = max_left_sum.max(left_sum);
            result = result.max(max_left_sum + max_right_sum);
        }

        buffer.clear();

        result
    }

    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let first_len = first_len as _;
        let second_len = second_len as _;
        let mut buffer = Vec::with_capacity(nums.len() - (first_len + second_len));
        let mut result = Self::helper(&nums, first_len, second_len, &mut buffer);

        if first_len != second_len {
            result = result.max(Self::helper(&nums, second_len, first_len, &mut buffer));
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        Self::max_sum_two_no_overlap(nums, first_len, second_len)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
