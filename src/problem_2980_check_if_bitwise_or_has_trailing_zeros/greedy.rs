pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        let mut iter = nums.iter().filter(|&num| num & 1 == 0);
        let mut found_one = || iter.next().is_some();

        found_one() && found_one()
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn has_trailing_zeros(nums: Vec<i32>) -> bool {
        Self::has_trailing_zeros(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
