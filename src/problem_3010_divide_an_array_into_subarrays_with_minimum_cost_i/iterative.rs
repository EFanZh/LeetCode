pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let nums = nums.into_iter().map(i32::cast_unsigned).collect::<Vec<_>>();
        let mut min_1 = u32::MAX;
        let mut min_2 = u32::MAX;

        for &num in &nums[1..] {
            if num < min_2 {
                (min_1, min_2) = if num < min_1 { (num, min_1) } else { (min_1, num) };
            }
        }

        (nums[0] + min_1 + min_2).cast_signed()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_cost(nums: Vec<i32>) -> i32 {
        Self::minimum_cost(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
