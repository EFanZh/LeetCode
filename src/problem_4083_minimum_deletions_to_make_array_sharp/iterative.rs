pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn min_deletions(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut prev = 0;

        for &x in &nums {
            result += i32::from(x == prev);
            prev = x;
        }

        result
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn min_deletions(nums: Vec<i32>) -> i32 {
        Self::min_deletions(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
