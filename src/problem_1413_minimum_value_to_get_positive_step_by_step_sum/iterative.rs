pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min_sum = i32::MAX;

        for num in nums {
            sum += num;
            min_sum = min_sum.min(sum);
        }

        (1 - min_sum).max(1)
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_start_value(nums: Vec<i32>) -> i32 {
        Self::min_start_value(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
