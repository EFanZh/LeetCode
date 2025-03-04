pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut result = 0_u32;
        let mut count = 0;

        for num in nums {
            sum += u64::from(num as u32);
            count += 1;
            result = result.max(sum.div_ceil(count) as _);
        }

        result as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimize_array_value(nums: Vec<i32>) -> i32 {
        Self::minimize_array_value(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
