pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut result_plus_31 = 31;
        let mut min_leading_zeros = 31;

        for num in nums {
            result_plus_31 += num.count_ones();
            min_leading_zeros = min_leading_zeros.min(num.leading_zeros());
        }

        (result_plus_31 - min_leading_zeros) as _
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_operations(nums: Vec<i32>) -> i32 {
        Self::min_operations(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
