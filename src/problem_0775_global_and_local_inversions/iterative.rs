pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        for (i, num) in (0..).zip(nums) {
            if !matches!(num - i, -1..=1) {
                return false;
            }
        }

        true
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        Self::is_ideal_permutation(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
