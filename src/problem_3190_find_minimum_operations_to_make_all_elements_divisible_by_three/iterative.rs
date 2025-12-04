pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.iter().map(|&num| i32::from(num.cast_unsigned() % 3 != 0)).sum()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn minimum_operations(nums: Vec<i32>) -> i32 {
        Self::minimum_operations(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
