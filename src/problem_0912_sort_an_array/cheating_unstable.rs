pub struct Solution;

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;

        nums.sort_unstable();

        nums
    }
}

// ------------------------------------------------------ snip ------------------------------------------------------ //

impl super::Solution for Solution {
    fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::sort_array(nums)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solution() {
        super::super::tests::run::<super::Solution>();
    }
}
