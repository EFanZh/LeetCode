pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        let mut odd = 0;

        for &num in &nums {
            odd += num & 1;
        }

        let even = nums.len() - odd.cast_unsigned() as usize;
        let mut result = nums;
        let (left, right) = result.split_at_mut(even);

        left.fill(0);
        right.fill(1);

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn transform_array(nums: Vec<i32>) -> Vec<i32> {
        Self::transform_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
